use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::RwLock;
use std::{fs, path::Path};
use thiserror::Error;
use clap::ValueEnum;

static CONFIG: once_cell::sync::OnceCell<RwLock<MisskeyConfig>> = once_cell::sync::OnceCell::new();

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MisskeyConfig {
  pub publish_tarball_instead_of_provide_repository_url: Option<bool>,
  pub setup_password: Option<String>,
  pub url: String,
  pub port: u64,
  pub db: DbConfig,
  pub db_replications: bool,
  pub db_slaves: Option<Vec<DbConfig>>,
  pub redis: RedisConfig,
  pub redis_for_pubsub: Option<RedisConfig>,
  pub redis_for_job_queue: Option<RedisConfig>,
  pub redis_for_timelines: Option<RedisConfig>,
  pub redis_for_reactions: Option<RedisConfig>,
  #[serde(rename = "fulltextSearch")]
  pub full_text_search: Option<FullTextSearch>,
  pub meilisearch: Option<MeilisearchConfig>,
  pub id: IdMethod,
  pub serde_for_backend: Option<SentryForBackendConfig>,
  pub serde_for_frontend: Option<SentryForFrontendConfig>,
  pub disable_hsts: Option<bool>,
  pub cluster_limit: Option<u64>,
  pub deliver_job_concurrency: Option<u64>,
  pub inbox_job_concurrency: Option<u64>,
  pub relationship_job_concurrency: Option<u64>,
  pub deliver_job_per_sec: Option<u64>,
  pub inbox_job_per_sec: Option<u64>,
  pub relationship_job_per_sec: Option<u64>,
  pub deliver_job_max_attempts: Option<u64>,
  pub inbox_job_max_attempts: Option<u64>,
  pub outgoing_address: Option<String>,
  pub outgoing_address_family: Option<OutgoingAddressFamily>,
  pub proxy: Option<String>,
  pub proxy_bypass_hosts: Option<Vec<String>>,
  pub proxy_smtp: Option<String>,
  pub media_proxy: Option<String>,
  pub proxy_remote_files: bool,
  pub video_thumbnail_generator: Option<String>,
  pub sign_to_activity_pub_get: bool,
  pub allowed_private_networks: Option<Vec<String>>,
  pub max_file_size: Option<u64>,
  pub pid_file: Option<String>,
  pub per_channel_max_note_cache_count: Option<u64>,
  pub per_user_notifications_max_count: Option<u64>,
  pub deactivate_antenna_threshold: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DbConfig {
  pub host: String,
  pub port: u64,
  pub db: String,
  pub user: String,
  pub pass: String,
  pub disable_cache: Option<bool>,
  pub extra: Option<DbExtraConfig>,
}

// TODO: pgのextraオプションへの追加対応 https://github.com/misskey-dev/misskey/issues/15108
#[derive(Debug, Serialize, Deserialize)]
pub struct DbExtraConfig {
  pub ssl: bool,
  pub statement_timeout: Option<u64>,
  pub query_timeout: Option<u64>,
  pub lock_timeout: Option<u64>,
  #[serde(flatten)]
  #[serde(skip_serializing)]
  _ignored: HashMap<String, serde_yml::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RedisConfig {
  pub host: String,
  pub port: u64,
  pub family: Option<RedisFamily>,
  pub pass: Option<String>,
  pub prefix: Option<String>,
  pub db: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RedisFamily {
  Both = 0,
  IPv4 = 4,
  IPv6 = 6,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct FullTextSearch {
  pub provider: FullTextSearchProvider,
}

// TODO: サメがtsvectorを使う場合の設定も追加
#[derive(Debug, Serialize, Deserialize, Clone, Copy, ValueEnum)]
#[serde(rename_all = "camelCase")]
pub enum FullTextSearchProvider {
  #[serde(rename = "sqlLike")]
  SqlLike,
  #[serde(rename = "sqlPgroonga")]
  SqlPgroonga,
  #[serde(rename = "meilisearch")]
  Meilisearch,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MeilisearchConfig {
  pub host: String,
  pub port: u64,
  pub api_key: String,
  pub ssl: bool,
  pub index: String,
  pub scope: Option<MeilisearchScope>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum MeilisearchScope {
  Local,
  Global,
  Custom(Vec<String>),
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum IdMethod {
  Aid,
  #[default]
  Aidx,
  Meid,
  Ulid,
  ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SentryForBackendConfig {
  pub enable_node_profiling: bool,
  pub options: Option<SentryOptions>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SentryForFrontendConfig {
  pub options: Option<SentryOptions>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SentryOptions {
  pub dsn: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutgoingAddressFamily {
  IPv4,
  IPv6,
  Dual,
}

#[derive(Debug, Error)]
pub enum ConfigError {
  #[error("Config not initialized")]
  ConfigNotInitialized,

  #[error("File not found: {path}")]
  ConfigFileNotFound { path: String },

  #[error("Failed to read config file: {0}")]
  ConfigFileReadError(#[from] std::io::Error),

  #[error("Failed to parse config file: {0}")]
  ConfigFileParseError(#[from] serde_yml::Error),

  #[error("Validation error: {0}")]
  ConfigValidationError(String),

  #[error("Initialization error")]
  ConfigInitializationError,
}

pub struct ServerConfig;

impl ServerConfig {
  pub fn init(config_path: &str) -> Result<(), ConfigError> {
    let config = Self::load_from_file(config_path)?;

    CONFIG
      .set(RwLock::new(config))
      .map_err(|_| ConfigError::ConfigInitializationError);

    Ok(())
  }

  fn load_from_file(config_path: &str) -> Result<MisskeyConfig, ConfigError> {
    let path = Path::new(config_path);

    if !path.exists() {
      return Err(ConfigError::ConfigFileNotFound {
        path: config_path.to_string(),
      });
    }

    let config_content = match fs::read_to_string(path) {
      Ok(content) => content,
      Err(e) => return Err(ConfigError::ConfigFileReadError(e)),
    };
    let config: MisskeyConfig = match serde_yml::from_str(&config_content) {
      Ok(cfg) => cfg,
      Err(e) => return Err(ConfigError::ConfigFileParseError(e)),
    };

    Ok(config)
  }

  pub fn get() -> Result<std::sync::RwLockReadGuard<'static, MisskeyConfig>, ConfigError> {
    let config = CONFIG.get().ok_or(ConfigError::ConfigNotInitialized)?;
    config.read().map_err(|_| ConfigError::ConfigNotInitialized)
  }

  pub fn get_id_method() -> Result<IdMethod, ConfigError> {
    Self::get().map(|cfg| cfg.id)
  }

  pub fn get_search_provider() -> Result<Option<FullTextSearchProvider>, ConfigError> {
    Self::get().map(|cfg| cfg.full_text_search.map(|provider| provider.provider))
  }

  pub fn get_meilisearch_config() -> Result<Option<MeilisearchConfig>, ConfigError> {
    Self::get().map(|cfg| cfg.meilisearch.clone())
  }
}
