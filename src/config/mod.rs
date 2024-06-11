use serde_derive::Deserialize;
use std::fs;
use std::net::{IpAddr, Ipv4Addr};
use toml;
/**
 * 读取配置文件
 * @author X.P
 */
//总配置文件
#[derive(Deserialize, PartialEq, Debug)]
pub struct Config {
    system: System,
}

//系统配置
#[derive(Deserialize, PartialEq, Debug)]
pub struct System {
    host: String,
    port: u16,
    env: String,
}
impl Config {
    pub fn new(filename: &str) -> System {
        let contents = match fs::read_to_string(filename) {
            Ok(c) => c,
            Err(e) => {
                panic!("无法加载 config.toml配置文件 {filename} {e}");
            }
        };

        let config: Config = match toml::from_str(&contents) {
            Ok(d) => d,
            Err(e) => {
                panic!("配置文件中没有 System 配置 {filename} {e} ");
            }
        };
        config.system
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_config_parse() {
        let data = Config::new("/Users/xp/Desktop/RustProjects/blog_new/src/config.toml");
        let data_01 = System {
            host: "127.0.0.1".to_string(),
            port: 8000,
            env: "debug".to_string(),
        };
        assert_eq!(data, data_01);
    }
}
