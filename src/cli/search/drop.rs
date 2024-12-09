use meilisearch_sdk::client::*;

use crate::config::load_config;

pub async fn drop(config_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config(config_path).unwrap();
    let meili_config = config.meilisearch.clone().unwrap();
    let host = meili_config.host;
    let port = meili_config.port;
    let ssl = meili_config.ssl;
    let api_key = meili_config.api_key;
    let index = meili_config.index;

    let host_url = format!("{}://{}:{}", if ssl { "https" } else { "http" }, host, port);
    let uid = format!("{}---notes", index);
    let client = Client::new(host_url, Some(api_key)).unwrap();
    let task = client.delete_index(uid).await.unwrap();
    println!("Dropped index: {}", task.status);

    Ok(())
}
