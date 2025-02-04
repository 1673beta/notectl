//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "system_webhook")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub id: String,
  #[sea_orm(column_name = "isActive")]
  pub is_active: bool,
  #[sea_orm(column_name = "updatedAt")]
  pub updated_at: DateTimeWithTimeZone,
  #[sea_orm(column_name = "latestSentAt")]
  pub latest_sent_at: Option<DateTimeWithTimeZone>,
  #[sea_orm(column_name = "latestStatus")]
  pub latest_status: Option<i32>,
  pub name: String,
  pub on: Vec<String>,
  pub url: String,
  pub secret: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(has_many = "super::abuse_report_notification_recipient::Entity")]
  AbuseReportNotificationRecipient,
}

impl Related<super::abuse_report_notification_recipient::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::AbuseReportNotificationRecipient.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}
