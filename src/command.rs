use crate::reply;

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

#[derive(Debug)]
pub enum TransferFileType {
    A, // 文本
    I, // 二进制
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

pub fn execute_command(cmd: &Cmd) -> (reply::SvrReply, Option<String>) {

    (reply::SvrReply::Unknown, None)
}
