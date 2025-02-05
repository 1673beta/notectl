//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "ad")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub id: String,
  #[sea_orm(column_name = "expiresAt")]
  pub expires_at: DateTimeWithTimeZone,
  pub place: String,
  pub priority: String,
  pub url: String,
  #[sea_orm(column_name = "imageUrl")]
  pub image_url: String,
  pub memo: String,
  pub ratio: i32,
  #[sea_orm(column_name = "startsAt")]
  pub starts_at: DateTimeWithTimeZone,
  #[sea_orm(column_name = "dayOfWeek")]
  pub day_of_week: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
