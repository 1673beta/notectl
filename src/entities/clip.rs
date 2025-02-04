//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "clip")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub id: String,
  #[sea_orm(column_name = "userId")]
  pub user_id: String,
  pub name: String,
  #[sea_orm(column_name = "isPublic")]
  pub is_public: bool,
  pub description: Option<String>,
  #[sea_orm(column_name = "lastClippedAt")]
  pub last_clipped_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(has_many = "super::clip_favorite::Entity")]
  ClipFavorite,
  #[sea_orm(has_many = "super::clip_note::Entity")]
  ClipNote,
  #[sea_orm(
    belongs_to = "super::user::Entity",
    from = "Column::UserId",
    to = "super::user::Column::Id",
    on_update = "NoAction",
    on_delete = "Cascade"
  )]
  User,
}

impl Related<super::clip_favorite::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::ClipFavorite.def()
  }
}

impl Related<super::clip_note::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::ClipNote.def()
  }
}

impl Related<super::user::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::User.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}
