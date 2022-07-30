use std::{ fs::File, path::Path, io::Read };
use std::collections::HashMap;

pub struct Config {
    pub ip: Option<String>,
    pub port: Option<String>,
    pub user: Option<String>,
    pub passwd: Option<String>,
    pub work_dir: Option<String>,
}

impl Config {
    fn new_default() -> Self {
        Self {
            ip: None,
            port: None,
            user: None,
            passwd: None,
            work_dir: None,
        }
    }
    pub fn new(cfg_path: &str) -> Self {
        let cfg_path = Path::new(cfg_path);
        let mut cfg = File::open(cfg_path).expect("Open file config failed.");
        let mut contents = String::new();
        let mut cfg_map = HashMap::new();

        cfg.read_to_string(&mut contents).expect("Read file config failed.");

        let contents: Vec<&str> = contents.as_str().split('\n').collect();
        for item in contents {
            if let Some((key, value)) = item.split_once('=') {
                // println!("key:{}, value:{}", key, value);
                cfg_map.insert(key.to_string(), value.to_string());
            }
        }

        let mut config = Config::new_default();
        if let Some(ip) = cfg_map.get("ip") { config.ip = Some(ip.to_string()); }
        if let Some(port) = cfg_map.get("port") { config.port = Some(port.to_string()); }
        if let Some(user) = cfg_map.get("user") { config.user = Some(user.to_string()); }
        if let Some(passwd) = cfg_map.get("passwd") { config.passwd = Some(passwd.to_string()); }
        if let Some(work_dir) = cfg_map.get("work_dir") { config.work_dir = Some(work_dir.to_string()); }

        config
    }
}
