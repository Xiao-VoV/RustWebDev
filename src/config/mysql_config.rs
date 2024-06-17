/**
 * 读取 Mysql 配置文件
 * @author X.P
 */
use serde_derive::Deserialize;
/**
url = "mysql://root:12345@127.0.0.1/blog"
max_connections = 50
min_connections = 2
connect_timeout = 8
acquire_timeout = 8
idle_timeout = 8
max_lifetime = 8
logging = true
logging_level = "info"
*/
#[derive(Deserialize, PartialEq, Debug)]
pub struct Config {
    pub mysql: Mysql,
}
impl Config {
    pub fn new() -> Self {
        Self {
            mysql: Mysql::new(),
        }
    }
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Mysql {
    url: String,
    max_connections: u32,
    min_connections: u32,
    connect_timeout: u64,
    acquire_timeout: u64,
    idle_timeout: u64,
    max_lifetime: u64,
    logging: bool,
    logging_level: String,
}
impl Mysql {
    pub fn new() -> Self {
        Self {
            url: "".to_string(),
            max_connections: 0,
            min_connections: 0,
            connect_timeout: 0,
            acquire_timeout: 0,
            idle_timeout: 0,
            max_lifetime: 0,
            logging: true,
            logging_level: "".to_string(),
        }
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn max_connections(&self) -> u32 {
        self.max_connections
    }

    pub fn min_connections(&self) -> u32 {
        self.min_connections
    }

    pub fn connect_timeout(&self) -> u64 {
        self.connect_timeout
    }

    pub fn acquire_timeout(&self) -> u64 {
        self.acquire_timeout
    }

    pub fn idle_timeout(&self) -> u64 {
        self.idle_timeout
    }

    pub fn max_lifetime(&self) -> u64 {
        self.max_lifetime
    }

    pub fn logging(&self) -> bool {
        self.logging
    }

    pub fn logging_level(&self) -> &str {
        &self.logging_level
    }
}
