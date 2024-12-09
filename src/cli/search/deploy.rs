use std::time::SystemTime;

use crate::config::{IdMethod, MeilisearchScope};
use crate::entities::prelude::*;
use meilisearch_sdk::client::*;
use meilisearch_sdk::settings::{PaginationSetting, Settings, TypoToleranceSettings};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

use crate::util::id::aid;
use crate::util::id::aidx;
use crate::util::id::ulid;
use crate::{config::load_config, db::postgres::connect_pg, entities::note};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Notes {
    pub id: String,
    pub created_at: SystemTime,
    pub user_id: String,
    pub user_host: Option<String>,
    pub channel_id: Option<String>,
    pub cw: Option<String>,
    pub text: Option<String>,
    pub tags: Vec<String>,
}

pub async fn deploy(config_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config(config_path).unwrap();
    let meili_config = config.meilisearch.clone().unwrap();
    let host = meili_config.host;
    let port = meili_config.port;
    let ssl = meili_config.ssl;
    let api_key = meili_config.api_key;
    let index = meili_config.index;
    let scope = meili_config.scope.unwrap();

    let pg_client = connect_pg(config_path).await.unwrap();

    let notes: Vec<note::Model> = match scope {
        MeilisearchScope::Local => {
            Note::find()
                .filter(note::Column::UserHost.is_null())
                .filter(
                    note::Column::Visibility
                        .eq("public")
                        .or(note::Column::Visibility.eq("home")),
                )
                .all(&pg_client)
                .await?
        }
        MeilisearchScope::Global => Note::find().all(&pg_client).await?,
        MeilisearchScope::Custom(hosts) => {
            let host = hosts.concat();
            Note::find()
                .filter(
                    note::Column::UserHost
                        .contains(host)
                        .or(note::Column::UserHost.is_null()),
                )
                .all(&pg_client)
                .await?
        }
    };

    let indexes: Vec<Notes> = match config.id {
        IdMethod::Aid => notes
            .iter()
            .map(|note| Notes {
                id: note.id.clone(),
                created_at: aid::parse(&note.id).unwrap(),
                user_id: note.user_id.clone(),
                user_host: note.user_host.clone(),
                channel_id: note.channel_id.clone(),
                cw: note.cw.clone(),
                text: note.text.clone(),
                tags: note.tags.clone(),
            })
            .collect(),
        IdMethod::Aidx => notes
            .iter()
            .map(|note| Notes {
                id: note.id.clone(),
                created_at: aidx::parse(&note.id).unwrap(),
                user_id: note.user_id.clone(),
                user_host: note.user_host.clone(),
                channel_id: note.channel_id.clone(),
                cw: note.cw.clone(),
                text: note.text.clone(),
                tags: note.tags.clone(),
            })
            .collect(),
        IdMethod::Ulid => notes
            .iter()
            .map(|note| Notes {
                id: note.id.clone(),
                created_at: ulid::parse(&note.id).unwrap(),
                user_id: note.user_id.clone(),
                user_host: note.user_host.clone(),
                channel_id: note.channel_id.clone(),
                cw: note.cw.clone(),
                text: note.text.clone(),
                tags: note.tags.clone(),
            })
            .collect(),
        IdMethod::Meid => {
            return Err("Meid is not supported yet".into());
        }
        IdMethod::ObjectId => {
            return Err("ObjectId is not supported yet".into());
        }
    };

    let host_url = format!("{}://{}:{}", if ssl { "https" } else { "http" }, host, port);
    let uid = format!("{}---notes", index);
    let client = Client::new(host_url, Some(api_key)).unwrap();

    let typotolerance = TypoToleranceSettings {
        enabled: Some(true),
        disable_on_attributes: None,
        disable_on_words: None,
        min_word_size_for_typos: None,
    };

    let pagination = PaginationSetting {
        max_total_hits: 10000,
    };

    let setting = Settings::new()
        .with_searchable_attributes(vec!["text", "cw"])
        .with_sortable_attributes(vec!["createdAt"])
        .with_filterable_attributes(vec!["createdAt", "userId", "userHost", "channelId", "tags"])
        .with_typo_tolerance(typotolerance)
        .with_pagination(pagination);

    let meili_index = client.index(uid.clone());
    // TODO: ここをtokio::spawnで並列処理にする
    let task_set = meili_index.set_settings(&setting).await.unwrap();
    let queue_set = client.wait_for_task(task_set, None, None).await.unwrap();
    let task_doc = meili_index
        .add_documents(&indexes, Some("id"))
        .await
        .unwrap();
    let queue_doc = client.wait_for_task(task_doc, None, None).await.unwrap();

    if queue_set.is_success() && queue_doc.is_success() {
        println!("Deployed index: {}", uid);
    } else {
        println!("Failed to deploy index");
    }

    Ok(())
}
