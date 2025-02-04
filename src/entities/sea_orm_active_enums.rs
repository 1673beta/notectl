//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "antenna_src_enum")]
pub enum AntennaSrcEnum {
  #[sea_orm(string_value = "all")]
  All,
  #[sea_orm(string_value = "home")]
  Home,
  #[sea_orm(string_value = "list")]
  List,
  #[sea_orm(string_value = "users")]
  Users,
  #[sea_orm(string_value = "users_blacklist")]
  UsersBlacklist,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(
  rs_type = "String",
  db_type = "Enum",
  enum_name = "instance_suspensionstate_enum"
)]
pub enum InstanceSuspensionstateEnum {
  #[sea_orm(string_value = "autoSuspendedForNotResponding")]
  AutoSuspendedForNotResponding,
  #[sea_orm(string_value = "goneSuspended")]
  GoneSuspended,
  #[sea_orm(string_value = "manuallySuspended")]
  ManuallySuspended,
  #[sea_orm(string_value = "none")]
  None,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(
  rs_type = "String",
  db_type = "Enum",
  enum_name = "meta_sensitivemediadetection_enum"
)]
pub enum MetaSensitivemediadetectionEnum {
  #[sea_orm(string_value = "all")]
  All,
  #[sea_orm(string_value = "local")]
  Local,
  #[sea_orm(string_value = "none")]
  None,
  #[sea_orm(string_value = "remote")]
  Remote,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(
  rs_type = "String",
  db_type = "Enum",
  enum_name = "meta_sensitivemediadetectionsensitivity_enum"
)]
pub enum MetaSensitivemediadetectionsensitivityEnum {
  #[sea_orm(string_value = "high")]
  High,
  #[sea_orm(string_value = "low")]
  Low,
  #[sea_orm(string_value = "medium")]
  Medium,
  #[sea_orm(string_value = "veryHigh")]
  VeryHigh,
  #[sea_orm(string_value = "veryLow")]
  VeryLow,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(
  rs_type = "String",
  db_type = "Enum",
  enum_name = "note_visibility_enum"
)]
pub enum NoteVisibilityEnum {
  #[sea_orm(string_value = "followers")]
  Followers,
  #[sea_orm(string_value = "home")]
  Home,
  #[sea_orm(string_value = "public")]
  Public,
  #[sea_orm(string_value = "specified")]
  Specified,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(
  rs_type = "String",
  db_type = "Enum",
  enum_name = "page_visibility_enum"
)]
pub enum PageVisibilityEnum {
  #[sea_orm(string_value = "followers")]
  Followers,
  #[sea_orm(string_value = "public")]
  Public,
  #[sea_orm(string_value = "specified")]
  Specified,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(
  rs_type = "String",
  db_type = "Enum",
  enum_name = "poll_notevisibility_enum"
)]
pub enum PollNotevisibilityEnum {
  #[sea_orm(string_value = "followers")]
  Followers,
  #[sea_orm(string_value = "home")]
  Home,
  #[sea_orm(string_value = "public")]
  Public,
  #[sea_orm(string_value = "specified")]
  Specified,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "relay_status_enum")]
pub enum RelayStatusEnum {
  #[sea_orm(string_value = "accepted")]
  Accepted,
  #[sea_orm(string_value = "rejected")]
  Rejected,
  #[sea_orm(string_value = "requesting")]
  Requesting,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "role_target_enum")]
pub enum RoleTargetEnum {
  #[sea_orm(string_value = "conditional")]
  Conditional,
  #[sea_orm(string_value = "manual")]
  Manual,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(
  rs_type = "String",
  db_type = "Enum",
  enum_name = "user_profile_followersVisibility_enum"
)]
pub enum UserProfileFollowersVisibilityEnum {
  #[sea_orm(string_value = "followers")]
  Followers,
  #[sea_orm(string_value = "private")]
  Private,
  #[sea_orm(string_value = "public")]
  Public,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(
  rs_type = "String",
  db_type = "Enum",
  enum_name = "user_profile_followingvisibility_enum"
)]
pub enum UserProfileFollowingvisibilityEnum {
  #[sea_orm(string_value = "followers")]
  Followers,
  #[sea_orm(string_value = "private")]
  Private,
  #[sea_orm(string_value = "public")]
  Public,
}
