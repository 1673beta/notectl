use meilisearch_sdk::documents::DocumentDeletionQuery;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, FromQueryResult, ModelTrait, QueryFilter, QuerySelect, TransactionTrait};
use serde::{Deserialize, Serialize};

use crate::{
  config::{load_config, IdMethod},
  consts::MeiliNotes,
  db::postgres::connect_pg,
  entities::{note, sea_orm_active_enums::NoteVisibilityEnum},
  util::id::{
    aid::gen_aid, aidx::gen_aidx, meid::gen_meid, objectid::gen_object_id, ulid::gen_ulid,
  },
};

#[derive(Debug, Serialize, Deserialize, FromQueryResult)]
struct NoteIdModel {
  id: String,
}

// ノートを削除するコマンド
// 1. 対象となるノートを検索する
// 2. meilisearchのconfigがnoneでない場合にまずmeilisearchにdelete投げる
// 3. postgresqlから削除する

pub async fn delete(
  config_path: &str,
  host: Option<&str>,
  days: u64,
  visibility: Option<Vec<NoteVisibilityEnum>>,
  no_reaction: bool,
  no_reply: bool,
  no_renote: bool,
  no_clipped: bool,
) -> Result<(), Box<dyn std::error::Error>> {
  // read config for judging whether meilisearch is enabled
  let config = load_config(config_path).unwrap();
  // connect to postgresql
  let pg_client = connect_pg(config_path).await.unwrap();
  // init logger
  tracing_subscriber::fmt()
    .with_max_level(tracing::Level::INFO)
    .init();

  let should_delete_from_meilisearch = visibility
    .as_ref()
    .map(|v| {
      !v.iter()
        .any(|vis| matches!(vis, NoteVisibilityEnum::Public | NoteVisibilityEnum::Home))
    })
    .unwrap_or(true);

  if should_delete_from_meilisearch {
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

      // create pg client for meilisearch
      let pg_client_meili = connect_pg(config_path).await.unwrap();
      let mut pg_query = note::Entity::find().select_only().column(note::Column::Id);
      // generate pg query for options
      if no_reaction {
        pg_query = pg_query.filter(note::Column::Reactions.ne("{}"));
      }
      if no_reply {
        pg_query = pg_query.filter(note::Column::RepliesCount.eq(0));
      }
      if no_renote {
        pg_query = pg_query.filter(note::Column::RenoteCount.eq(0));
      }
      if no_clipped {
        pg_query = pg_query.filter(note::Column::ClippedCount.eq(0));
      }

      let ids: Vec<String> = pg_query
        .into_model::<NoteIdModel>()
        .all(&pg_client_meili)
        .await?
        .into_iter()
        .map(|note| note.id)
        .collect();

      // createdAtをfilterの条件としてmeilisearchから削除する
      let index = client.index(uid);
      let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
      let date = now - days * 24 * 60 * 60;

      let created_at = format!("createdAt < {}", date);
      let mut filters = vec![created_at.clone()];
      if let Some(host) = host {
        filters.push(format!("userHost = \"{}\"", host));
      } else {
        filters.push("userHost IS NOT NULL".to_string());
      }

      if !ids.is_empty() {
        let id_filter = format!("id IN [{}]", ids.join(","));
        filters.push(id_filter);
      }
      let filter = filters.join(" AND ");

      // do query
      let task = DocumentDeletionQuery::new(&index)
        .with_filter(&filter)
        .execute::<MeiliNotes>()
        .await?;

      task.wait_for_completion(&client, None, None).await.unwrap();
      tracing::info!("Meilisearch delete task completed");
      pg_client_meili.close().await?;
    }
  } else {
    tracing::info!("Meilisearch is not enabled or specified visibility is not including public or home, skipping");
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

  let now = std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)
    .unwrap()
    .as_millis() as u64;
  let date = now - days * 24 * 60 * 60;

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

  if let Some(visibility) = visibility {
    query = query.filter(note::Column::Visibility.is_in(visibility));
  }

  if no_reaction {
    query = query.filter(note::Column::Reactions.ne("{}"));
  }

  if no_reply {
    query = query.filter(note::Column::RepliesCount.eq(0));
  }

  if no_renote {
    query = query.filter(note::Column::RenoteCount.eq(0));
  }

  if no_clipped {
    query = query.filter(note::Column::ClippedCount.eq(0));
  }

  // delete from database
  let notes = query.all(&txn).await?;
  let mut reply_ids = std::collections::HashSet::new();
  for note in &notes {
    if let Some(reply_id) = &note.reply_id {
      reply_ids.insert(reply_id.clone());
    }
  }

  if !reply_ids.is_empty() {
    let reply_ids_vec: Vec<String> = reply_ids.into_iter().collect();

    let reply_targets = note::Entity::find()
    .filter(note::Column::Id.is_in(reply_ids_vec))
    .all(&txn)
    .await?;

    for target in reply_targets {
      let mut target_model: note::ActiveModel = target.clone().into();

      let new_count = if target.replies_count > 0 { target.replies_count - 1 } else { 0 };
      target_model.replies_count = sea_orm::ActiveValue::Set(new_count);

      target_model.update(&txn).await?;
    }
  }

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
