pub enum SvrReply {
    OpenDataConnection,                                // 150
    CorrectCommand(String),                            // 200
    FTPSvrReady,                                       // 220
    QuitFtpSvr,                                        // 221
    CloseDataConnection,                               // 226
    ChangeToPasvMode,                                  // 227
    LoginSvrSucceed(String),                           // 230
    ChangeDirectory(String, String),                   // 250
    CurrentDirectory(String),                          // 257
    RequestPasswd(String),                             // 331
    RequestUser,                                       // 332
    UnableOpenDataConnection,                          // 425
    AbandonConnection,                                 // 426
    InvalidCommand(String),                            // 500
    WrongCommandParameter(String, String),             // 501
    LoginSvrFailed(String),                            // 530
    NonExistFile(String),                              // 550
    Unknown,                                           // 1000
}

pub fn generate_reply_message(reply: SvrReply) -> String {
    let reply = match reply {
        SvrReply::OpenDataConnection => "150 Open data connection.".to_string(),
        SvrReply::CorrectCommand(cmd) => "200 ".to_string() + &cmd + " command successful.",
        SvrReply::FTPSvrReady => "220 FTP server ready to work.".to_string(),
        SvrReply::QuitFtpSvr => "221 Quit FTP server.".to_string(),
        SvrReply::CloseDataConnection => "226 Close data connection.".to_string(),
        SvrReply::ChangeToPasvMode => "227 Change to PASV mode.".to_string(),
        SvrReply::LoginSvrSucceed(user) => "230 User ".to_string() + &user + " logged in.",
        SvrReply::ChangeDirectory(cmd, dir) => { 
            "250 ".to_string() + &cmd + " command successful." + "\"" + &dir + "\" is current directory."
        },
        SvrReply::CurrentDirectory(dir) => "257 ".to_string() + "\"" + &dir + "\" is current directory.",
        SvrReply::RequestPasswd(user) => "331 Password required for ".to_string() + &user + ".",
        SvrReply::RequestUser => "332 User required for logging in.".to_string(),
        SvrReply::UnableOpenDataConnection => "425 Unable to open data connection.".to_string(),
        SvrReply::AbandonConnection => "426 Abandon the connection.".to_string(),
        SvrReply::InvalidCommand(cmd) => "500 ".to_string() + &cmd + " command is invalid.",
        SvrReply::WrongCommandParameter(cmd, para) => {
            "501 ".to_string() + &cmd + " command was given a wrong parameter \'" + &para + "\'."
        },
        SvrReply::LoginSvrFailed(user) => "530 User ".to_string() + &user + " logging in failed.",
        SvrReply::NonExistFile(file) => "550 File \'".to_string() + &file + "\' does not exist.",
        SvrReply::Unknown => "1000 Unknown reply".to_string(),
    };

    reply + "\r\n"
}
