// 閉鎖などで存在しなくなったリモートアカウントを削除する
use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, QueryFilter, QuerySelect, TransactionTrait};

use crate::db::postgres::connect_pg;
use crate::entities::sea_orm_active_enums::InstanceSuspensionstateEnum;
use crate::entities::{instance, prelude::*, user};

pub async fn cull(config_path: &str) -> Result<(), Box<dyn std::error::Error>> {
  let pg_client = connect_pg(config_path).await?;
  let txn = pg_client.begin().await?;
  // need to output logs
  tracing_subscriber::fmt()
    .with_max_level(tracing::Level::INFO)
    .init();

  let gone_hosts = Instance::find()
    .select_only()
    .filter(instance::Column::SuspensionState.eq(InstanceSuspensionstateEnum::GoneSuspended))
    .all(&pg_client)
    .await?;

  let host_name: Vec<String> = gone_hosts.iter().map(|i| i.host.clone()).collect();

  let accounts = User::find()
    .filter(user::Column::Host.is_in(host_name))
    .all(&pg_client)
    .await?;

  for account in accounts {
    account.delete(&txn).await?;
  }
  txn.commit().await?;
  Ok(())
}
