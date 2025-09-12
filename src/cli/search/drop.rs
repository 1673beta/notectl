use crate::services::meilisearch::MeilisearchService;

pub async fn drop() -> Result<(), Box<dyn std::error::Error>> {
  let service = MeilisearchService::init()?;
  service.drop().await?;

  Ok(())
}
