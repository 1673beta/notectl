use meilisearch_sdk::client::*;

use crate::config::load_config;

pub async fn health(config_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config(config_path).unwrap();
    let meili_config = config.meilisearch.clone().unwrap();
    let host = meili_config.host;
    let port = meili_config.port;
    let ssl = meili_config.ssl;
    let api_key = meili_config.api_key;

    let host_url = format!("{}://{}:{}", if ssl { "https" } else { "http" }, host, port);
    let client = Client::new(host_url, Some(api_key)).unwrap();
    let is_healthy = client.is_healthy().await;
    let health = client.health().await.unwrap().status.to_string();

    println!("Is Meilisearch Healthy?: {}", is_healthy);
    println!("Health Status: {}", health);

    Ok(())
}
