use meilisearch_sdk::documents::DocumentDeletionQuery;
use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, QueryFilter, TransactionTrait};

use crate::{
    config::{load_config, IdMethod},
    consts::MeiliNotes,
    db::postgres::connect_pg,
    entities::note,
    util::id::{
        aid::gen_aid, aidx::gen_aidx, meid::gen_meid, objectid::gen_object_id, ulid::gen_ulid,
    },
};

// ノートを削除するコマンド
// 1. 対象となるノートを検索する
// 2. meilisearchのconfigがnoneでない場合にまずmeilisearchにdelete投げる
// 3. postgresqlから削除する

pub async fn delete(
    config_path: &str,
    host: Option<&str>,
    days: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    // read config for judging whether meilisearch is enabled
    let config = load_config(config_path).unwrap();
    // connect to postgresql
    let pg_client = connect_pg(config_path).await.unwrap();
    // init logger
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // do if meilisearch is enabled
    if let Some(meili_config) = config.meilisearch {
        // get meilisearch config
        let meili_host = meili_config.host;
        let port = meili_config.port;
        let ssl = meili_config.ssl;
        let api_key = meili_config.api_key;
        let m_index = meili_config.index;
        tracing::info!("Meilisearch is enabled");

        // format meilisearch index name
        let uid = format!("{}---notes", m_index);

        // parse meilisearch url for meilisearch_sdk
        let url = format!(
            "{}://{}:{}",
            if ssl { "https" } else { "http" },
            meili_host,
            port
        );
        // create meilisearch client
        let client = meilisearch_sdk::client::Client::new(url, Some(api_key)).unwrap();
        // createdAtをfilterの条件としてmeilisearchから削除する
        let index = client.index(uid);
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let date = now - days * 24 * 60 * 60;
        if date == 0 {
            tracing::error!("Cannot specify today or future date");
            std::process::exit(3);
        }

        let created_at = format!("createdAt < {}", date);
        let base_filter = created_at.clone();

        let task = if let Some(host) = host {
            let user_host = format!("userHost = \"{}\"", host);
            let query = format!("{} AND {}", base_filter, user_host);
            DocumentDeletionQuery::new(&index)
                .with_filter(&query)
                .execute::<MeiliNotes>()
                .await
                .unwrap()
        } else {
            DocumentDeletionQuery::new(&index)
                .with_filter(&base_filter)
                .execute::<MeiliNotes>()
                .await
                .unwrap()
        };

        task.wait_for_completion(&client, None, None).await.unwrap();
        tracing::info!("Meilisearch delete task completed");
    } else {
        tracing::info!("Meilisearch is not enabled");
    }

    // start transaction
    let txn = pg_client.begin().await?;

    // generate delete query
    let mut query = note::Entity::find();
    if let Some(host) = host {
        query = query.filter(note::Column::UserHost.eq(host));
    } else {
        query = query.filter(note::Column::UserHost.is_not_null());
    }
    // TODO: ここでIDから日付を取得してlteにsubstrを入れてクエリを生成する
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    let date = now - days * 24 * 60 * 60;
    if date == 0 {
        tracing::error!("Cannot specify today or future date");
        std::process::exit(3);
    }

    match config.id {
        IdMethod::Aid => {
            let id = gen_aid(date).unwrap();
            query = query.filter(note::Column::Id.lte(&id[0..8]));
        }
        IdMethod::Aidx => {
            let id = gen_aidx(date).unwrap();
            query = query.filter(note::Column::Id.lte(&id[0..8]));
        }
        IdMethod::Meid => {
            let id = gen_meid(date);
            query = query.filter(note::Column::Id.lte(&id[0..12]));
        }
        IdMethod::ObjectId => {
            let id = gen_object_id(date);
            query = query.filter(note::Column::Id.lte(&id[0..8]));
        }
        IdMethod::Ulid => {
            let id = gen_ulid(date);
            query = query.filter(note::Column::Id.lte(&id[0..10]));
        }
    }

    // delete from database
    let notes = query.all(&txn).await?;
    let chunk_size = 100;
    for chunk in notes.chunks(chunk_size) {
        let futures = chunk.iter().map(|note| note.clone().delete(&txn));
        futures::future::join_all(futures).await;
        tracing::info!("Deleted {} notes", chunk.len());
    }

    // commit transaction
    txn.commit().await?;
    Ok(())
}
