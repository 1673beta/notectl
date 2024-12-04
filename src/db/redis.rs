use redis::aio::MultiplexedConnection;
use redis::{Client, RedisError, RedisResult};

use crate::config;

pub async fn connect() -> RedisResult<MultiplexedConnection> {
    let config = config::load_config().unwrap();
    let redis_url = format!(
        "redis://{}:{}/",
        config.redis.host, config.redis.port,
    );
    let client = Client::open(redis_url).unwrap();
    let con = client.get_multiplexed_tokio_connection().await.unwrap();
    Ok(con)
}