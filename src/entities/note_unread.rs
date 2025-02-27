//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.6

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "note_unread")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub id: String,
  #[sea_orm(column_name = "userId")]
  pub user_id: String,
  #[sea_orm(column_name = "noteId")]
  pub note_id: String,
  #[sea_orm(column_name = "noteUserId")]
  pub note_user_id: String,
  #[sea_orm(column_name = "isSpecified")]
  pub is_specified: bool,
  #[sea_orm(column_name = "isMentioned")]
  pub is_mentioned: bool,
  #[sea_orm(column_name = "noteChannelId")]
  pub note_channel_id: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(
    belongs_to = "super::note::Entity",
    from = "Column::NoteId",
    to = "super::note::Column::Id",
    on_update = "NoAction",
    on_delete = "Cascade"
  )]
  Note,
  #[sea_orm(
    belongs_to = "super::user::Entity",
    from = "Column::UserId",
    to = "super::user::Column::Id",
    on_update = "NoAction",
    on_delete = "Cascade"
  )]
  User,
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
