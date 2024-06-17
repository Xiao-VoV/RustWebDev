use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};

pub use sea_orm;

use crate::config::logger_config::get_log_level;
use crate::config::{get_by_section_name, SectionName};

// 数据库连接
pub async fn connect() -> Result<DatabaseConnection, DbErr> {
    let mysql_config = if let SectionName::MysqlCfg(config) = get_by_section_name("mysql") {
        config
    } else {
        panic!("无法连接数据库！")
    };
    let db_logging_level = mysql_config.logging_level();
    let logging_level = get_log_level(db_logging_level);

    let mut opt = ConnectOptions::new(mysql_config.url());
    opt.max_connections(mysql_config.max_connections())
        .min_connections(mysql_config.min_connections())
        .connect_timeout(Duration::from_secs(mysql_config.connect_timeout()))
        .acquire_timeout(Duration::from_secs(mysql_config.acquire_timeout()))
        .idle_timeout(Duration::from_secs(mysql_config.idle_timeout()))
        .max_lifetime(Duration::from_secs(mysql_config.max_lifetime()))
        .sqlx_logging(mysql_config.logging())
        .sqlx_logging_level(logging_level);

    let db = Database::connect(opt);

    db.await
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    pub async fn db_connect_test() {
        let db = connect().await.unwrap();
        assert!(db.ping().await.is_ok());
        db.clone().close().await;
        // assert!(matches!(db.ping().await, Err(DbErr::ConnectionAcquire)));
    }
}
