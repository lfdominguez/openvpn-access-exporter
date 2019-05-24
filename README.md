Prometheus OpenVPN Access Exporter
==================================
[![Build Status](https://travis-ci.com/lfdominguez/openvpn-access-exporter.svg?branch=master)](https://travis-ci.com/lfdominguez/openvpn-access-exporter)
[![Build status](https://ci.appveyor.com/api/projects/status/m97n73atmvnj5lox?svg=true)](https://ci.appveyor.com/project/lfdominguez/openvpn-access-exporter)

This is my first Rust program. Take the SQLite db `log.db` of OpenVPN Access Server and expose this metrics to Prometheus.

Metrics:
  * Session duration with `openvpn_user_duration`
  * Bytes downloaded on session with `openvpn_user_bytes_in`
  * Bytes uploaded on session with`openvpn_user_bytes_out`

with labels:
  * `session_id`
  * `node`
  * `username`
  * `common_name`
  * `real_ip`
  * `vpn_ip`

CLI reference:
```
USAGE:
    openvpn-access-exporter [OPTIONS] --file <file>

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --file <file>    SQLite log file (log.db)
    -h, --host <host>    Address where to expose http server [default: 0.0.0.0]
    -p, --port <port>    Host port to expose http server [default: 9185]
```
