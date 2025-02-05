//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "channel_favorite")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub id: String,
  #[sea_orm(column_name = "channelId")]
  pub channel_id: String,
  #[sea_orm(column_name = "userId")]
  pub user_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(
    belongs_to = "super::channel::Entity",
    from = "Column::ChannelId",
    to = "super::channel::Column::Id",
    on_update = "NoAction",
    on_delete = "Cascade"
  )]
  Channel,
  #[sea_orm(
    belongs_to = "super::user::Entity",
    from = "Column::UserId",
    to = "super::user::Column::Id",
    on_update = "NoAction",
    on_delete = "Cascade"
  )]
  User,
}

impl Related<super::channel::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Channel.def()
  }
}

impl Related<super::user::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::User.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}
