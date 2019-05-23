Prometheus OpenVPN Access Exporter
==================================
This is my first Rust program. Take the SQLite db `log.db` of OpenVPN Access Server and expose some metrics to Prometheus.

This is a exporter for:
  * duration
  * bytes_in
  * bytes_out

with labels:
  * session_id
  * node
  * username
  * common_name
  * real_ip
  * vpn_ip

Usage:
```
openvpn-access-exporter --file <file>
```

and listen on `9185` port.

TODO:
-----
- [ ] Allow change exposed port
- [ ] Allow change exposed interface
