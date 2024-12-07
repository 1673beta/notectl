use inkjet::{formatter, theme::{vendored, Theme}, Highlighter, Language};
use meilisearch_sdk::{client::*, indexes::IndexesResults};
use termcolor::{ColorChoice, StandardStream};

use crate::config::load_config;

pub async fn list(config_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config(config_path).unwrap();
    let meili_config = config.meilisearch.clone().unwrap();
    let host = meili_config.host;
    let port = meili_config.port;
    let ssl = meili_config.ssl;
    let api_key = meili_config.api_key;

    let host_url: String = format!("{}://{}:{}", if ssl { "https"} else { "http"}, host, port);
    let client: Client = Client::new(host_url, Some(api_key)).unwrap();
    let index = client.list_all_indexes_raw().await.unwrap();
    let json = serde_json::to_string_pretty(&index).unwrap();
    let mut highlighter = Highlighter::new();
    let theme = Theme::from_helix(vendored::TERM16_DARK).unwrap();
    let stream = StandardStream::stdout(ColorChoice::AlwaysAnsi);
    let formatter = formatter::Terminal::new(theme, stream);
    let colored = highlighter.highlight_to_string(Language::Json, &formatter,json)?;
    println!("{}", colored);
    Ok(())
}