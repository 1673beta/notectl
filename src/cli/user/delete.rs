// アカウントをID指定で削除する

use sea_orm::{ColumnTrait, Condition, EntityTrait, ModelTrait, QueryFilter, TransactionTrait};

use crate::consts::UserIdentifier;
use crate::db::postgres::connect_pg;
use crate::entities::{prelude::*, user};

pub async fn delete(
  config_path: &str,
  id: Option<Vec<&str>>,
  user: Option<Vec<UserIdentifier>>,
) -> Result<(), Box<dyn std::error::Error>> {
  let pg_client = connect_pg(config_path).await?;
  let txn = pg_client.begin().await?;
  // need to output logs
  tracing_subscriber::fmt()
    .with_max_level(tracing::Level::INFO)
    .init();

  if id.is_none() && user.is_none() {
    tracing::error!("Either username or id is required.");
    std::process::exit(1);
  }

  let mut query = User::find();

  if let Some(users) = user {
    for user in users {
      query = query.filter(
        Condition::all()
          .add(user::Column::Username.contains(user.username))
          .add(user::Column::Host.contains(user.host)),
      )
    }
  }

  if let Some(id) = id {
    let user_id = id.iter().map(|i| i.to_string()).collect::<Vec<String>>();
    query = query.filter(user::Column::Id.is_in(user_id));
  }

  let accounts = query.all(&txn).await?;

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
