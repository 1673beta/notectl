use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{
    config::{load_config, IdMethod}, db::postgres::connect_pg, entities::{note, prelude::*}
};

pub async fn delete(
    config_path: &str,
    before: u64,
    host: Option<&str>,
    reply: bool,
    quote: bool,
) {
    let pg_client = connect_pg(config_path).await.unwrap();
    let config = load_config(config_path).unwrap();
    let id_type = config.id;

    let mut query = Note::find();

    // hostはOptionなので、Optionの中身がある場合のみフィルタリングする
    if let Some(host) = host {
        query = query.filter(note::Column::UserHost.eq(host));
    }

    if reply {
        todo!();
    }

    if quote {
        query = query.filter(note::Column::Text.is_not_null().or(note::Column::Cw.is_not_null()));
    }

    let id = match id_type {
        IdMethod::Aid => {}
        IdMethod::Aidx => {}
        IdMethod::Meid => {}
        IdMethod::Ulid => {}
        IdMethod::ObjectId => {}
    };

    let notes: Vec<note::Model> = query.all(&pg_client).await.unwrap();
}
