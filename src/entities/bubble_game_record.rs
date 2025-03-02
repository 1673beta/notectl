//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.6

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "bubble_game_record")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub id: String,
  #[sea_orm(column_name = "userId")]
  pub user_id: String,
  #[sea_orm(column_name = "seededAt")]
  pub seeded_at: DateTimeWithTimeZone,
  pub seed: String,
  #[sea_orm(column_name = "gameVersion")]
  pub game_version: i32,
  #[sea_orm(column_name = "gameMode")]
  pub game_mode: String,
  pub score: i32,
  #[sea_orm(column_type = "JsonBinary")]
  pub logs: Json,
  #[sea_orm(column_name = "isVerified")]
  pub is_verified: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(
    belongs_to = "super::user::Entity",
    from = "Column::UserId",
    to = "super::user::Column::Id",
    on_update = "NoAction",
    on_delete = "Cascade"
  )]
  User,
}

impl Related<super::user::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::User.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}
