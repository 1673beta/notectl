use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

use crate::db::postgres::connect_pg;
use crate::entities::{instance, prelude::*, sea_orm_active_enums};

pub async fn unsuspend(config_path: &str, url: &str) -> Result<(), Box<dyn std::error::Error>> {
  let pg_client = connect_pg(config_path).await.unwrap();

  let host = Instance::find()
    .filter(instance::Column::Host.eq(url))
    .one(&pg_client)
    .await
    .unwrap();
  if host.is_none() {
    println!("Host not found");
    return Err("Host not found".into());
  }
  let mut host: instance::ActiveModel = host.unwrap().into();
  host.suspension_state = Set(sea_orm_active_enums::InstanceSuspensionstateEnum::None);
  host.is_not_responding = Set(false);
  host.update(&pg_client).await?;
  Ok(())
}
