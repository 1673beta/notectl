//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.6

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "blocking")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub id: String,
  #[sea_orm(column_name = "blockeeId")]
  pub blockee_id: String,
  #[sea_orm(column_name = "blockerId")]
  pub blocker_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(
    belongs_to = "super::user::Entity",
    from = "Column::BlockerId",
    to = "super::user::Column::Id",
    on_update = "NoAction",
    on_delete = "Cascade"
  )]
  User2,
  #[sea_orm(
    belongs_to = "super::user::Entity",
    from = "Column::BlockeeId",
    to = "super::user::Column::Id",
    on_update = "NoAction",
    on_delete = "Cascade"
  )]
  User1,
}

impl ActiveModelBehavior for ActiveModel {}
