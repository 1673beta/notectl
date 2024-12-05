use std::{fmt, fs, path::Path};

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

impl fmt::Display for MisskeyConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "MisskeyConfig {{ publish_tarball_instead_of_provide_repository_url: {:?}, url: {}, port: {}, db: {}, db_replications: {}, db_slaves: {:?}, redis: {}, redis_for_pubsub: {:?}, redis_for_job_queue: {:?}, redis_for_timelines: {:?}, meilisearch: {:?}, id: {} }}",
            self.publish_tarball_instead_of_provide_repository_url, self.url, self.port, self.db, self.db_replications, self.db_slaves, self.redis, self.redis_for_pubsub, self.redis_for_job_queue, self.redis_for_timelines, self.meilisearch, self.id)
    }
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

impl fmt::Display for DbConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "DbConfig {{ host: {}, port: {}, db: {}, user: {}, pass: {}, disable_cache: {:?}, extra: {:?} }}",
            self.host, self.port, self.db, self.user, self.pass, self.disable_cache, self.extra)
    }
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

impl fmt::Display for RedisConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "RedisConfig {{ host: {}, port: {}, family: {:?}, pass: {:?}, prefix: {:?}, db: {:?} }}",
            self.host, self.port, self.family, self.pass, self.prefix, self.db)
    }
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

impl fmt::Display for MeilisearchConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "MeilisearchConfig {{ host: {}, port: {}, api_key: {}, ssl: {}, index: {}, scope: {:?} }}",
            self.host, self.port, self.api_key, self.ssl, self.index, self.scope)
    }
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

impl fmt::Display for IdMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Aid => writeln!(f, "aid"),
            Self::Aidx => writeln!(f, "aidx"),
            Self::Meid => writeln!(f, "meid"),
            Self::Ulid => writeln!(f, "ulid"),
            Self::ObjectId => writeln!(f, "object_id"),
        }
    }
}

pub fn load_config() -> Result<MisskeyConfig, Box<dyn std::error::Error>> {
    let config_path = Path::new(".config/default.yml");
    let config_content = fs::read_to_string(config_path).expect("Failed to read config file");
    let config: MisskeyConfig =
        serde_yml::from_str(&config_content).expect("Failed to parse config file");
    Ok(config)
}
