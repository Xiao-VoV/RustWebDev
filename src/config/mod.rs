pub mod logger_config;
pub mod mysql_config;
pub mod system_config;

use once_cell::sync::Lazy;
use serde::de::DeserializeOwned;
use serde_derive::Deserialize;
use std::fmt::Debug;
use std::fs;
use std::sync::Mutex;

// //未使用
// #[derive(Deserialize, PartialEq, Debug)]
// pub struct Config<T> {
//     config: T,
// }
static GLOBAL_CONF_FILE: Lazy<Mutex<String>> = Lazy::new(|| {
    Mutex::new(String::from(
        "/Users/xp/Desktop/RustProjects/blog_new/src/config.toml",
    ))
});

static GLOBAL_CONF: Lazy<Mutex<String>> = Lazy::new(|| {
    if let Ok(v) = GLOBAL_CONF_FILE.lock() {
        match fs::read_to_string(v.clone().as_str()) {
            Ok(c) => return Mutex::new(c),
            Err(e) => {
                panic!("无法加载 config.toml配置文件 {e}");
            }
        }
    }
    panic!("无法加载配置文件")
});

pub fn section<T: DeserializeOwned>(config: &mut T) {
    if let Ok(v) = GLOBAL_CONF.lock() {
        *config = toml::from_str(v.as_str()).unwrap();
    }
}

//无效
// pub fn section_config<T: for<'a> Deserialize<'a>>(config: &mut Config<T>) {
//     if let Ok(v) = GLOBAL_CONF.lock() {
//         *config = toml::from_str(v.as_str()).unwrap();
//     }
// }

pub enum SectionName {
    System(system_config::System),
    Logger(logger_config::Logger),
    MysqlCfg(mysql_config::Mysql),
}
//通过 section 名称获取反序列化的 struct 对象
pub fn get_by_section_name(section_name: &str) -> SectionName {
    match section_name {
        "system" => {
            let mut config = system_config::Config::new();
            section(&mut config);
            return SectionName::System(config.system);
        }
        "mysql" => {
            let mut config = mysql_config::Config::new();
            section(&mut config);
            return SectionName::MysqlCfg(config.mysql);
        }
        _ => {
            let mut config = system_config::Config::new();
            section(&mut config);
            return SectionName::System(config.system);
        }
    }
}

#[cfg(test)]
mod test {

    use crate::config::section;

    use super::*;
    #[test]
    pub fn test_config_parse() {
        let mut config = get_by_section_name("system");
        let data_01 = system_config::System {
            host: "127.0.0.1".to_string(),
            port: 8000,
            env: "debug".to_string(),
        };
        let x = if let SectionName::System(y) = config {
            y
        } else {
            panic!();
        };
        assert_eq!(x, data_01);
    }
}
