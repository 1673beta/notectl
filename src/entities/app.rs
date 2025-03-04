//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.6

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "app")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub id: String,
  #[sea_orm(column_name = "userId")]
  pub user_id: Option<String>,
  pub secret: String,
  pub name: String,
  pub description: String,
  pub permission: Vec<String>,
  #[sea_orm(column_name = "callbackUrl")]
  pub callback_url: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(has_many = "super::access_token::Entity")]
  AccessToken,
  #[sea_orm(has_many = "super::auth_session::Entity")]
  AuthSession,
  #[sea_orm(
    belongs_to = "super::user::Entity",
    from = "Column::UserId",
    to = "super::user::Column::Id",
    on_update = "NoAction",
    on_delete = "SetNull"
  )]
  User,
}

impl Related<super::access_token::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::AccessToken.def()
  }
}

impl Related<super::auth_session::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::AuthSession.def()
  }
}

impl Related<super::user::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::User.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}
