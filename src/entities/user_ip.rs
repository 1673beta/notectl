//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.6

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "user_ip")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: i32,
  #[sea_orm(column_name = "createdAt")]
  pub created_at: DateTimeWithTimeZone,
  #[sea_orm(column_name = "userId")]
  pub user_id: String,
  pub ip: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
