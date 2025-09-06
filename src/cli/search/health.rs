use crate::services::meilisearch::MeilisearchService;

pub async fn health() -> Result<(), Box<dyn std::error::Error>> {
  let service = MeilisearchService::init()?;
  service.health().await?;
  Ok(())
}
