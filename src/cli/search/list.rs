use meilisearch_sdk::client::*;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

use crate::config::load_config;

pub async fn list(config_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config(config_path).unwrap();
    let meili_config = config.meilisearch.clone().unwrap();
    let host = meili_config.host;
    let port = meili_config.port;
    let ssl = meili_config.ssl;
    let api_key = meili_config.api_key;

    let host_url: String = format!("{}://{}:{}", if ssl { "https" } else { "http" }, host, port);
    let client: Client = Client::new(host_url, Some(api_key)).unwrap();
    let index = client.list_all_indexes_raw().await.unwrap();
    let json = serde_json::to_string_pretty(&index).unwrap();

    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let syntax = ps.find_syntax_by_extension("json").unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
    for line in LinesWithEndings::from(&json) {
        let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ps).unwrap();
        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
        print!("{}", escaped);
    }
    Ok(())
}
