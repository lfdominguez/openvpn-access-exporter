// Will create an exporter with a single metric that will randomize the value
// of the metric everytime the exporter is called.

extern crate prometheus;
extern crate env_logger;
extern crate clap;

#[macro_use] extern crate log;

use env_logger::{
    Builder,
    Env
};

use prometheus::{
    __register_gauge_vec,
    opts,
    register_gauge_vec,
    register_counter
};
use prometheus_exporter::{
    FinishedUpdate,
    PrometheusExporter,
};
use clap::{App, Arg};
use std::net::SocketAddr;
use sqlite::State;

fn main() {
    let flags = App::new("openvpn-access-exporter")
        .version("0.1")
        .about("Prometheus exporter for OpenVPN Access Server")
        .author("Luis Felipe Domínguez Vega <ldominguezvega@gmail.com>")
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .help("SQLite log file (log.db)")
            .required(true)
            .takes_value(true)
        )
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .help("Host port to expose http server")
            .required(false)
            .takes_value(true)
            .default_value("9185")
        )
        .arg(Arg::with_name("host")
            .short("h")
            .long("host")
            .help("Address where to expose http server")
            .required(false)
            .takes_value(true)
            .default_value("0.0.0.0")
        )
        .get_matches();

    let ovpn_log = flags.value_of("file").unwrap();
    let expose_port = flags.value_of("port").unwrap();
    let expose_host = flags.value_of("host").unwrap();

    // Setup logger with default level info so we can see the messages from
    // prometheus_exporter.
    Builder::from_env(Env::default().default_filter_or("info")).init();

    info!("Using file: {}", ovpn_log);

    // Parse address used to bind exporter to.
    let addr_raw = expose_host.to_owned() + ":" + expose_port;
    let addr: SocketAddr = addr_raw.parse().expect("can not parse listen addr");

    // Start exporter.
    let (request_receiver, finished_sender) = PrometheusExporter::run_and_notify(addr);

    let access_counter = register_counter!("access_counter", "Requests counter")
        .expect("Can't create counter access_counter");

    let label_vector = [
        "session_id", 
        "node",
        "username",
        "common_name",
        "real_ip",
        "vpn_ip"
    ];

    // Create metric
    let bytes_in = register_gauge_vec!("openvpn_user_bytes_in", "Bytes in", &label_vector)
        .expect("can not create gauge openvpn_user_bytes_in");
    let bytes_out = register_gauge_vec!("openvpn_user_bytes_out", "Bytes out", &label_vector)
        .expect("can not create gauge openvpn_user_bytes_out");
    let duration = register_gauge_vec!("openvpn_user_duration", "Connection duration", &label_vector)
        .expect("can not create gauge openvpn_user_duration");

    let connection = sqlite::open(ovpn_log).unwrap();

    loop {
        // Will block until exporter receives http request.
        request_receiver.recv().unwrap();

        access_counter.inc();

        let mut statement = connection
            .prepare("SELECT session_id, node, username, common_name, real_ip, vpn_ip, duration, bytes_in, bytes_out FROM log WHERE active = 1")
            .unwrap();

        while let State::Row = statement.next().unwrap() {
            let label_values = [
                &statement.read::<String>(0).unwrap()[..],
                &statement.read::<String>(1).unwrap()[..],
                &statement.read::<String>(2).unwrap()[..],
                &statement.read::<String>(3).unwrap()[..],
                &statement.read::<String>(4).unwrap()[..],
                &statement.read::<String>(5).unwrap()[..]
            ];

            duration.with_label_values(&label_values).set(statement.read::<f64>(5).unwrap());
            bytes_in.with_label_values(&label_values).set(statement.read::<f64>(6).unwrap());
            bytes_out.with_label_values(&label_values).set(statement.read::<f64>(7).unwrap());
        }

        // Notify exporter that all metrics have been updated so the caller client can
        // receive a response.
        finished_sender.send(FinishedUpdate).unwrap();
    }
}