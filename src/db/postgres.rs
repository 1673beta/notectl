use sea_orm::{Database, DatabaseConnection, DbConn, DbErr};

use crate::config;

pub async fn connect_pg() -> Result<DbConn, DbErr> {
    let config = config::load_config().unwrap();
    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.db.user, config.db.pass, config.db.host, config.db.port, config.db.db
    );

    let db: DatabaseConnection = Database::connect(&db_url).await?;
    Ok(db)
}

pub async fn close_pg(db: DbConn) -> Result<(), DbErr> {
    db.close().await?;
    Ok(())
}
