// アカウントをまとめて削除する

use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, QueryFilter, TransactionTrait};

use crate::db::postgres::connect_pg;
use crate::entities::{prelude::*, user};

pub async fn prune(
    config_path: &str,
    host: Option<&str>,
    zero_note: bool,
    no_follow: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let pg_client = connect_pg(config_path).await?;
    let txn = pg_client.begin().await?;
    // need to output logs
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let mut query = User::find();

    if let Some(host) = host {
        query = query.filter(user::Column::Host.eq(host));
    } else {
        query = query.filter(user::Column::Host.is_not_null());
    }

    if zero_note {
        query = query.filter(user::Column::NotesCount.eq(0));
    }

    if no_follow {
        query = query.filter(
            user::Column::FollowersCount
                .eq(0)
                .and(user::Column::FollowingCount.eq(0)),
        );
    }

    let users = query.all(&pg_client).await?;

    for user in users {
        user.delete(&txn).await?;
    }
    txn.commit().await?;
    Ok(())
}
