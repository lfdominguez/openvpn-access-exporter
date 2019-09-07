use prometheus::{GaugeVec, Counter};

lazy_static! {
  
  pub static ref ACCESS_COUNTER: Counter = register_counter!("access_counter", "Requests counter")
    .unwrap();
  pub static ref BYTES_IN: GaugeVec = register_gauge_vec!("openvpn_user_bytes_in", "Bytes in",
    &["session_id", "node", "username", "common_name", "real_ip", "vpn_ip"])
    .expect("can not create gauge openvpn_user_bytes_in");
  pub static ref BYTES_OUT: GaugeVec = register_gauge_vec!("openvpn_user_bytes_out", "Bytes out",
    &["session_id", "node", "username", "common_name", "real_ip", "vpn_ip"])
    .expect("can not create gauge openvpn_user_bytes_out");
  pub static ref DURATION: GaugeVec = register_gauge_vec!("openvpn_user_duration", "Connection duration",
    &["session_id", "node", "username", "common_name", "real_ip", "vpn_ip"])
    .expect("can not create gauge openvpn_user_duration");
}
