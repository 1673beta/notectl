// meilisearch関連
use meilisearch_sdk::settings::{PaginationSetting, Settings, TypoToleranceSettings};
use meilisearch_sdk::{client::*, errors};
use thiserror::Error;

use crate::configs::server::{MeilisearchConfig, ServerConfig};

#[derive(Error, Debug)]
pub enum MeilisearchServiceError {
  #[error("Meilisearch client error")]
  MeilisearchClientError(#[from] meilisearch_sdk::errors::Error),

  #[error("Database connection error")]
  DatabaseError(#[from] sea_orm::DbErr),

  #[error("Configuration error: {0}")]
  ConfigError(String),
}

pub struct MeilisearchService {
  client: Client,
  config: MeilisearchConfig,
}

impl MeilisearchService {
  pub fn init() -> Result<Self, MeilisearchServiceError> {
    let config = ServerConfig::get_meilisearch_config().map_err(|e| MeilisearchServiceError::ConfigError(e.to_string()))?.unwrap();

    let host_url = format!(
      "{}:{}//{}",
      if config.ssl { "https" } else { "http" },
      config.host,
      config.port
    );

    let client = Client::new(host_url, Some(config.api_key.clone()))?;

    Ok(Self {
      client,
      config,
    })
  }

  // TODO: deploy, drop, health, list, dump, snapshotに関する関数を追加する
  // deployの機能のうち、設定に関する部分はこっち、SQLで対象のノート探すのは別途SqlQueryServiceに移管する
  // listの文字装飾はどう考えても冗長なので廃止する
  // dumpはdumpを取る機能を用意する、できたらdumpを取り込む機能も入れたい
  // snapshotも同様
  
  pub async fn drop(&self) -> Result<String, MeilisearchServiceError> {
    let uid = format!("{}---notes", self.config.index);
    let task = self.client.delete_index(uid).await?;
    println!("Dropped index: {}", task.status);
    Ok(task.status)
  }

  pub async fn health(&self) -> Result<(), MeilisearchServiceError> {
    let is_healthy = self.client.is_healthy().await;
    let health = self.client.health().await.unwrap().status.to_string();

    println!("Is Meilisearch Healthy?: {}", is_healthy);
    println!("Health Status: {}", health);

    Ok(())
  }

  // TODO: 文字装飾は今後も別なところで発生する可能性があるからその時にはsyntect関連を別なモジュールやサービスに移す
  pub async fn list(&self) -> Result<(), MeilisearchServiceError> {
    let client = &self.client;
    let index = client.list_all_indexes_raw().await.unwrap();
    let json = serde_json::to_string_pretty(&index).unwrap();

    let ps = syntect::parsing::SyntaxSet::load_defaults_newlines();
    let ts = syntect::highlighting::ThemeSet::load_defaults();

    let syntax = ps.find_syntax_by_extension("json").unwrap();
    let mut h = syntect::easy::HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);

    for line in syntect::util::LinesWithEndings::from(&json) {
      let ranges: Vec<(syntect::highlighting::Style, &str)> = h.highlight_line(line, &ps).unwrap();
      let escaped = syntect::util::as_24_bit_terminal_escaped(&ranges[..], true);
      print!("{}", escaped);
    }
    Ok(())
  }
}
