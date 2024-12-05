use std::{fs, path::Path};

use inkjet::{formatter, theme::{vendored, Theme}, Highlighter, Language};
use termcolor::{ColorChoice, StandardStream};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MisskeyConfig {
    pub publish_tarball_instead_of_provide_repository_url: Option<bool>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct MeilisearchConfig {
    pub host: String,
    pub port: u64,
    pub api_key: String,
    pub ssl: bool,
    pub index: String,
    pub scope: Option<MeilisearchScope>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MeilisearchScope {
    Local,
    Global,
    Custom(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IdMethod {
    Aid,
    Aidx,
    Meid,
    Ulid,
    ObjectId,
}

impl Default for IdMethod {
    fn default() -> Self {
        IdMethod::Aidx
    }
}

pub fn load_config() -> Result<MisskeyConfig, Box<dyn std::error::Error>> {
    let config_path = Path::new(".config/default.yml");
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