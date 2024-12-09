use std::{fs, path::Path};

use inkjet::{formatter, theme::{vendored, Theme}, Highlighter, Language};
use termcolor::{ColorChoice, StandardStream};
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize)]
pub struct DbExtraConfig {
    ssl: bool,
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

#[derive(Debug, Default, Serialize, Deserialize)]
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
    Dual
}

pub fn load_config(config_path: &str) -> Result<MisskeyConfig, Box<dyn std::error::Error>> {
    let config_path = Path::new(config_path);
    let config_content = fs::read_to_string(config_path).expect("Failed to read config file");
    let config: MisskeyConfig =
        serde_yml::from_str(&config_content).expect("Failed to parse config file");
    Ok(config)
}

pub fn print_config(config: &MisskeyConfig) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(config)?;
    let mut highlighter = Highlighter::new();
    let theme = Theme::from_helix(vendored::BASE16_TERMINAL).unwrap();
    let stream = StandardStream::stdout(ColorChoice::AlwaysAnsi);
    let formatter = formatter::Terminal::new(theme, stream);
    let colored = highlighter.highlight_to_string(Language::Json, &formatter, json)?;
    println!("{}", colored);
    Ok(())
}