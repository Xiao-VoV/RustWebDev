/**
 * 读取配置文件
 * @author X.P
 */
/*
//总配置文件
use config::{Config, File};
use once_cell::sync::Lazy;
use std::path::Path;
use std::sync::Mutex;

static GLOBAL_CONF_FILE: Lazy<Mutex<String>> = Lazy::new(|| {
    Mutex::new(String::from(
        "/Users/xp/Desktop/RustProjects/blog_new/src/config.toml",
    ))
});

static GLOBAL_CONF: Lazy<Config> = Lazy::new(|| {
    if let Ok(v) = GLOBAL_CONF_FILE.lock() {
        if Path::new(v.clone().as_str()).exists() {
            return Config::builder()
                .add_source(File::from(Path::new(v.clone().as_str())))
                .build()
                .unwrap();
        }
    }
    panic!("没有配置文件！")
});

*/
/*********************************/
use serde_derive::Deserialize;
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
impl System {
    pub fn new() -> Self {
        Self {
            host: "".to_string(),
            port: 0,
            env: "0".to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::config::section;

    use super::*;
    #[test]
    pub fn test_config_parse() {
        let mut config: Config = Config {
            system: System::new(),
        };
        section(&mut config);
        let data_01 = System {
            host: "127.0.0.1".to_string(),
            port: 8000,
            env: "debug".to_string(),
        };
        assert_eq!(config.system, data_01);
    }
}
