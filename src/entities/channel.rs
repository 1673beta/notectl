//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.6

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "channel")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub id: String,
  #[sea_orm(column_name = "lastNotedAt")]
  pub last_noted_at: Option<DateTimeWithTimeZone>,
  #[sea_orm(column_name = "userId")]
  pub user_id: Option<String>,
  pub name: String,
  pub description: Option<String>,
  #[sea_orm(column_name = "bannerId")]
  pub banner_id: Option<String>,
  #[sea_orm(column_name = "notesCount")]
  pub notes_count: i32,
  #[sea_orm(column_name = "usersCount")]
  pub users_count: i32,
  #[sea_orm(column_name = "pinnedNoteIds")]
  pub pinned_note_ids: Vec<String>,
  pub color: String,
  #[sea_orm(column_name = "isArchived")]
  pub is_archived: bool,
  #[sea_orm(column_name = "isSensitive")]
  pub is_sensitive: bool,
  #[sea_orm(column_name = "allowRenoteToExternal")]
  pub allow_renote_to_external: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(has_many = "super::channel_favorite::Entity")]
  ChannelFavorite,
  #[sea_orm(has_many = "super::channel_following::Entity")]
  ChannelFollowing,
  #[sea_orm(has_many = "super::channel_note_pining::Entity")]
  ChannelNotePining,
  #[sea_orm(
    belongs_to = "super::drive_file::Entity",
    from = "Column::BannerId",
    to = "super::drive_file::Column::Id",
    on_update = "NoAction",
    on_delete = "SetNull"
  )]
  DriveFile,
  #[sea_orm(has_many = "super::note::Entity")]
  Note,
  #[sea_orm(
    belongs_to = "super::user::Entity",
    from = "Column::UserId",
    to = "super::user::Column::Id",
    on_update = "NoAction",
    on_delete = "SetNull"
  )]
  User,
}

impl Related<super::channel_favorite::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::ChannelFavorite.def()
  }
}

impl Related<super::channel_following::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::ChannelFollowing.def()
  }
}

impl Related<super::channel_note_pining::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::ChannelNotePining.def()
  }
}

impl Related<super::drive_file::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::DriveFile.def()
  }
}

impl Related<super::note::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Note.def()
  }
}

impl Related<super::user::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::User.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}
