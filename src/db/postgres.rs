use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbConn, DbErr};

use crate::configs::server::ServerConfig;

pub async fn connect_pg(config_path: &str) -> Result<DbConn, DbErr> {
  let config = ServerConfig::get().map_err(|e| DbErr::Custom(e.to_string()))?;
  let db_url = format!(
    "postgres://{}:{}@{}:{}/{}",
    config.db.user, config.db.pass, config.db.host, config.db.port, config.db.db
  );

  let mut opt = ConnectOptions::new(&db_url);
  opt
    .sqlx_logging(true)
    .sqlx_logging_level(log::LevelFilter::Info)
    .min_connections(2)
    .max_connections(100);

  // TODO: 接続に失敗したときにリトライするようにする
  let db = Database::connect(opt).await?;
  Ok(db)
}
