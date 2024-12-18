// 閉鎖などで存在しなくなったリモートアカウントを削除する
use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, QueryFilter, QuerySelect};
use tokio::task;

use crate::entities::sea_orm_active_enums::InstanceSuspensionstateEnum;
use crate::entities::{instance, prelude::*, user};
use crate::db::postgres::connect_pg;

pub async fn cull(config_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let pg_client = connect_pg(config_path).await?;
    let gone_hosts = Instance::find()
        .select_only()
        .filter(instance::Column::SuspensionState.eq(InstanceSuspensionstateEnum::GoneSuspended))
        .all(&pg_client)
        .await?;

    let host_name: Vec<String> = gone_hosts.iter().map(|i| i.host.clone()).collect();

    let accounts = User::find()
        .filter(
            user::Column::Host.is_in(host_name),
        )
        .all(&pg_client)
        .await?;

    for account in accounts {
        let db = pg_client.clone();
        account.delete(&db).await?;
    }
    Ok(())
}