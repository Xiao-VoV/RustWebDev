mod logger_config;
mod system_config;

use once_cell::sync::Lazy;
use serde::de::DeserializeOwned;
use std::fs;
use std::sync::Mutex;

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
