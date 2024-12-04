mod config;
mod db;
mod entities;

#[tokio::main]
async fn main() {
    let config = config::load_config();
    println!("{:?}", config);
    let db = db::postgres::connect_pg().await;
    match db {
        Ok(db) => {
            println!("Connected to database");
            db.close().await.unwrap();
        }
        Err(e) => {
            println!("Failed to connect to database: {:?}", e);
        }
    }
    let redis = db::redis::connect().await;
    match redis {
        Ok(_) => {
            println!("Connected to Redis");
        }
        Err(e) => {
            println!("Failed to connect to Redis: {:?}", e);
        }
    }
}
