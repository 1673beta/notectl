use crate::services::meilisearch::MeilisearchService;

pub async fn list() -> Result<(), Box<dyn std::error::Error>> {
  let service = MeilisearchService::init()?;
  service.list().await?;
  Ok(())
}
