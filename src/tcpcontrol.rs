use std::net::{ TcpListener, TcpStream };
use std::io::prelude::*;
use crate::{ config, reply, command };

pub fn start_tcp_listener(cfg: &config::Config) {
    let mut addr = String::new();
    if let Some(ip) = &cfg.ip { addr += ip; }
    if let Some(port) = &cfg.port { addr += ":"; addr += port; }
    // println!("addr: {}", addr);
    let listener = TcpListener::bind(addr).expect("TCP bind failed.");
    println!("start tcp listener successfully");
    println!("waiting for connection...");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                answer_reply(&mut stream, reply::SvrReply::FTPSvrReady);
                handle_connection(stream);
            }
            Err(e) => panic!("encountered IO error: {e}"),
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut recv_buf = [0; 32];
    let size = stream.read(&mut recv_buf[..]).expect("read error");
    if let Ok(request) = String::from_utf8(Vec::from(&recv_buf[..size])) {
        let request = request.trim();
        println!("{:?}", request);
        if let Some((cmd, args)) = request.split_once(' ') {
            if let Some(cmd) = command::generate_command(cmd, args) {
                let (reply, exec_result) = command::execute_command(&cmd);
                if let Some(exec_result) = exec_result {
                    answer_execute_result(&mut stream, &exec_result);
                    answer_reply(&mut stream, reply);
                }

            } else {
                answer_reply(&mut stream, reply::SvrReply::InvalidCommand(request.to_string()));
            }
        }
    }
}

fn answer_execute_result(stream: &mut TcpStream, result: &str) -> usize {
    stream.write(result.as_bytes()).expect("send result failed.")
}

fn answer_reply(stream: &mut TcpStream, reply: reply::SvrReply) -> usize {
    let reply_msg = reply::generate_reply_message(reply);
    stream.write(reply_msg.as_bytes()).expect("send reply failed.")
}
