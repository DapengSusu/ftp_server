use std::net::TcpStream;
use std::io::prelude::*;
use std::process::Command;

use crate::{ reply, config, command, tcpcontrol::start_tcp_connection };

pub enum Cmd {
    User(String),             // 登录用户
    Pass(String),             // 登录密码
    Port(String, u8, u8),     // 主动模式，端口号由两部分构成：port=port1*256+port2
    Pasv,                     // 被动模式
    Cd(String),               // 改变当前服务器的文件目录
    Pwd,                      // 显示当前服务器的工作目录
    List(String),             // 列出文件目录
    Mkd(String),              // 创建目录
    Rmd(String),              // 删除目录
    Dele(String),             // 删除文件
    Type(TransferFileType),   // 设置传输文件类型（文本或二进制）
    Retr(String),             // 从服务器上下载文件
    Stor(String),             // 向服务器上上传文件
    Abor,                     // 放弃前一个命令
    Quit,                     // 退出登录
    Invalid,                  // 未知指令
}

pub enum TransferFileType {
    A, // 文本
    I, // 二进制
}

pub struct CmdController {
    username: Option<String>,
    password: Option<String>,
    login_succeed: bool,
    tcp_data_stream: Option<TcpStream>,
    cmd_history: Vec<String>,
}

impl CmdController {
    pub fn new() -> Self {
        Self {
            username: None,
            password: None,
            login_succeed: false,
            tcp_data_stream: None,
            cmd_history: Vec::new()
        }
    }

    pub fn username(&self) -> Option<&str> {
        if let Some(username) = &self.username {
            Some(username)
        } else {
            None
        }
    }

    pub fn password(&self) -> Option<&str> {
        if let Some(password) = &self.password {
            Some(password)
        } else {
            None
        }
    }

    pub fn login_succeed(&self) -> bool { self.login_succeed }

    pub fn tcp_data_stream(&self) -> Option<&TcpStream> {
        if let Some(tcp_data_stream) = &self.tcp_data_stream {
            Some(tcp_data_stream)
        } else {
            None
        }
    }

    pub fn cmd_history(&self) -> Vec<&str> {
        let mut cmd_vec: Vec<&str> = Vec::new();
        for cmd in &self.cmd_history {
            cmd_vec.push(cmd);
        }

        cmd_vec
    }
}

pub fn generate_command(cmd: &str, args: &str) -> Option<Cmd> {
    match cmd {
        "USER" => Some(Cmd::User(args.to_string())),
        "PASS" => Some(Cmd::Pass(args.to_string())),
        "PORT" => { // args like: 127,0,0,1,30,31
            let addr: Vec<&str> = args.rsplitn(3, ',').collect();
            if addr.len() != 3 { return None;}

            let ip = addr[2].replace(",", ".");
            if let Ok(port1) = addr[1].parse::<u8>() {
                if let Ok(port2) = addr[0].parse::<u8>() {
                    return Some(Cmd::Port(ip, port1, port2));
                }
            }
            None
        }
        "LIST" => Some(Cmd::List(args.to_string())),

        _ => None,
    }
}

pub fn execute_command(cmd: &Cmd, cfg: &config::Config, user_info: &mut command::CmdController)
    -> (reply::SvrReply, Option<String>)
{
    match &cmd {
        Cmd::User(user) => {
            if let Some(cfg_user) = &cfg.user {
                if cfg_user == user {
                    user_info.username = Some(user.to_string());
                    return (reply::SvrReply::RequestPasswd(user.to_string()), None);
                } else {
                    return (reply::SvrReply::RequestUser,
                        Some("Non-existent user: ".to_string() + user));
                }
            }
            (reply::SvrReply::RequestUser, Some("Can't find user setting.".to_string()))
        },
        Cmd::Pass(passwd) => {
            if let Some(user) = &user_info.username {
                if let Some(cfg_passwd) = &cfg.passwd {
                    if cfg_passwd == passwd {
                        user_info.password = Some(passwd.to_string());
                        user_info.login_succeed = true;
                        return (reply::SvrReply::LoginSvrSucceed(user.to_string()), None);
                    } else {
                        return (reply::SvrReply::RequestPasswd(user.to_string()),
                            Some("Wrong password, current user: ".to_string() + user));
                    }
                }
                return (reply::SvrReply::RequestPasswd(user.to_string()),
                    Some("Can't find passwd setting.".to_string()));
            }
            (reply::SvrReply::RequestUser, Some("Enter username first.".to_string()))
        },
        Cmd::Port(ip, port1, port2) => {
            if user_info.login_succeed {
                let port  = *port1 as u16 * 256 + *port2 as u16;
                let addr = ip.to_string() + ":" + &port.to_string();
                let (reply, data_stream) = start_tcp_connection(&addr);
                
                user_info.tcp_data_stream = data_stream;

                return (reply, None);
            }
            (reply::SvrReply::RequestUser, Some("Log in ftp server first.".to_string()))
        },
        Cmd::List(dir) => {
            if user_info.login_succeed {
                if let Some(data_stream) = &mut user_info.tcp_data_stream {
                    let (exec_status, exec_result) = exec_cmd("ls", dir);
                    let mut reply = reply::SvrReply::OpenDataConnection;
                    if !exec_status {
                        reply = reply::SvrReply::NonExistFile(dir.to_string());
                    }
                    data_stream.write(exec_result.as_bytes()).expect("send result failed.");

                    return (reply, None)
                }
            }
            (reply::SvrReply::RequestUser, Some("Log in ftp server first.".to_string()))
        },

        _ => (reply::SvrReply::Unknown, None)
    }
}

fn exec_cmd(cmd: &str, arg: &str) -> (bool, String) {
    let output = Command::new(cmd)
            .arg(arg)
            .output()
            .expect("failed to execute process.");

    (output.status.success(), String::from_utf8(output.stdout).expect("parse output failed."))
}
