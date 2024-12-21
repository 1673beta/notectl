use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbConn, DbErr};

use crate::config;

pub async fn connect_pg(config_path: &str) -> Result<DbConn, DbErr> {
    let config = config::load_config(config_path).unwrap();
    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.db.user, config.db.pass, config.db.host, config.db.port, config.db.db
    );

    let mut opt = ConnectOptions::new(&db_url);
    opt.sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);

    let db: DatabaseConnection = Database::connect(opt).await?;
    Ok(db)
}
