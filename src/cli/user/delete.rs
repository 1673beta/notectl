// アカウントをID指定で削除する

use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, QueryFilter, TransactionTrait};

use crate::db::postgres::connect_pg;
use crate::entities::{prelude::*, user};

pub async fn delete(config_path: &str, id: Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
  let pg_client = connect_pg(config_path).await?;
  let txn = pg_client.begin().await?;
  // need to output logs
  tracing_subscriber::fmt()
    .with_max_level(tracing::Level::INFO)
    .init();

  let user_id = id.iter().map(|i| i.to_string()).collect::<Vec<String>>();

  let accounts = User::find()
    .filter(user::Column::Id.is_in(user_id))
    .all(&pg_client)
    .await?;

  for account in accounts {
    if account.host.is_none() {
      return Err("This account is not remote account.".into());
    } else if account.is_root {
      return Err("This account is root account.".into());
    } else {
      account.delete(&txn).await?;
    }
  }
  txn.commit().await?;
  Ok(())
}
