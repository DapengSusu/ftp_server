use ftp_server::{ tcpcontrol, config };

fn main() {
    let cfg = config::Config::new("config.ini");

    tcpcontrol::start_tcp_listener(&cfg);
}
