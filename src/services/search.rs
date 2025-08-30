// SQL検索とmeilisearch関連
use meilisearch_sdk::settings::{PaginationSetting, Settings, TypoToleranceSettings};
use meilisearch_sdk::{client::*, errors};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SearchServiceError {
  #[error("Meilisearch client error")]
  MeilisearchClientError(#[from] meilisearch_sdk::errors::Error),

  #[error("Database connection error")]
  DatabaseError(#[from] sea_orm::DbErr),
}
