//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "reversi_game")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[sea_orm(column_name = "startedAt")]
    pub started_at: Option<DateTimeWithTimeZone>,
    #[sea_orm(column_name = "user1Id")]
    pub user1_id: String,
    #[sea_orm(column_name = "user2Id")]
    pub user2_id: String,
    #[sea_orm(column_name = "user1Ready")]
    pub user1_ready: bool,
    #[sea_orm(column_name = "user2Ready")]
    pub user2_ready: bool,
    pub black: Option<i32>,
    #[sea_orm(column_name = "isStarted")]
    pub is_started: bool,
    #[sea_orm(column_name = "isEnded")]
    pub is_ended: bool,
    #[sea_orm(column_name = "winnerId")]
    pub winner_id: Option<String>,
    #[sea_orm(column_name = "surrenderedUserId")]
    pub surrendered_user_id: Option<String>,
    #[sea_orm(column_type = "JsonBinary")]
    pub logs: Json,
    pub map: Vec<String>,
    pub bw: String,
    #[sea_orm(column_name = "isLlotheo")]
    pub is_llotheo: bool,
    #[sea_orm(column_name = "canPutEverywhere")]
    pub can_put_everywhere: bool,
    #[sea_orm(column_name = "loopedBoard")]
    pub looped_board: bool,
    #[sea_orm(column_type = "JsonBinary", nullable)]
    pub form1: Option<Json>,
    #[sea_orm(column_type = "JsonBinary", nullable)]
    pub form2: Option<Json>,
    pub crc32: Option<String>,
    #[sea_orm(column_name = "timeoutUserId")]
    pub timeout_user_id: Option<String>,
    #[sea_orm(column_name = "endedAt")]
    pub ended_at: Option<DateTimeWithTimeZone>,
    #[sea_orm(column_name = "timeLimitForEachTurn")]
    pub time_limit_for_each_turn: i16,
    #[sea_orm(column_name = "noIrregularRules")]
    pub no_irregular_rules: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::User2Id",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    User2,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::User1Id",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    User1,
}

impl ActiveModelBehavior for ActiveModel {}