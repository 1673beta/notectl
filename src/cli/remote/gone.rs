use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

use crate::{
  db::postgres::connect_pg,
  entities::{instance, prelude::*},
};

pub async fn gone(config_path: &str, url: &str) -> Result<(), Box<dyn std::error::Error>> {
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
  host.suspension_state =
    Set(crate::entities::sea_orm_active_enums::InstanceSuspensionstateEnum::GoneSuspended);
  host.is_not_responding = Set(true);
  host.update(&pg_client).await?;
  Ok(())
}
