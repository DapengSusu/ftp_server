use ftp_server::{ tcpcontrol, config };
use std::path::Path;

fn main() {
    let cfg_path = Path::new("config1.ini");
    let cfg = config::Config::new(cfg_path);

    tcpcontrol::start_tcp_listener(&cfg);
}
