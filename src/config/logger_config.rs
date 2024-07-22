use humantime;
use serde_derive::Deserialize;
use std::time::SystemTime;

#[derive(Deserialize, PartialEq, Debug)]
pub struct Config {
    logger: Logger,
}

//日志配置
#[derive(Deserialize, PartialEq, Debug)]
pub struct Logger {
    level: String,
    debug: bool,
}

impl Logger {
    pub fn new() -> Self {
        Self {
            level: "".to_string(),
            debug: false,
        }
    }
}

// 获取日志等级
pub fn get_log_level(name: &str) -> log::LevelFilter {
    match name {
        "debug" => log::LevelFilter::Debug,
        "error" => log::LevelFilter::Error,
        "info" => log::LevelFilter::Info,
        "trace" => log::LevelFilter::Trace,
        "warn" => log::LevelFilter::Warn,
        _ => log::LevelFilter::Info,
    }
}

fn setup_logger() -> Result<(), fern::InitError> {
    let mut f = fern::Dispatch::new().format(|out, message, record| {
        out.finish(format_args!(
            "[{} {} {}] {}",
            humantime::format_rfc3339_seconds(SystemTime::now()),
            record.level(),
            record.target(),
            message
        ))
    });
    let mut config = Config {
        logger: Logger::new(),
    };
    super::section(&mut config);
    let logging_level = get_log_level(config.logger.level.as_str());
    f = f.level(logging_level);

    if config.logger.debug {
        f = f.chain(std::io::stdout());
    }

    f.chain(fern::log_file("guda.log")?).apply()?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use log::{debug, info, trace, warn};

    #[test]
    pub fn log_config_parse() {
        setup_logger();

        info!("Hello, world!");
        warn!("Warning!");
        debug!("Now exiting.");
    }
}
