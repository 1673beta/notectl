use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

use crate::db::postgres::connect_pg;
use crate::entities::{instance, prelude::*, sea_orm_active_enums};

pub async fn suspend(config_path: &str, url: &str) -> Result<(), Box<dyn std::error::Error>> {
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

  let suspension_state = host.clone().unwrap().suspension_state;
  if suspension_state == sea_orm_active_enums::InstanceSuspensionstateEnum::GoneSuspended {
    return Err("Host is already gone".into());
  } else if suspension_state == sea_orm_active_enums::InstanceSuspensionstateEnum::ManuallySuspended
  {
    return Err("Host is already suspended".into());
  }

  let mut host: instance::ActiveModel = host.unwrap().into();
  tokio::spawn(async move {
    host.suspension_state =
      Set(sea_orm_active_enums::InstanceSuspensionstateEnum::ManuallySuspended);
    host.is_not_responding = Set(true);
    host.update(&pg_client).await.unwrap();
  })
  .await
  .unwrap();
  Ok(())
}
