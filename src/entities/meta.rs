//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.6

use super::sea_orm_active_enums::MetaSensitivemediadetectionEnum;
use super::sea_orm_active_enums::MetaSensitivemediadetectionsensitivityEnum;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "meta")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub id: String,
  pub name: Option<String>,
  pub description: Option<String>,
  #[sea_orm(column_name = "maintainerName")]
  pub maintainer_name: Option<String>,
  #[sea_orm(column_name = "maintainerEmail")]
  pub maintainer_email: Option<String>,
  #[sea_orm(column_name = "disableRegistration")]
  pub disable_registration: bool,
  pub langs: Vec<String>,
  #[sea_orm(column_name = "hiddenTags")]
  pub hidden_tags: Vec<String>,
  #[sea_orm(column_name = "blockedHosts")]
  pub blocked_hosts: Vec<String>,
  #[sea_orm(column_name = "mascotImageUrl")]
  pub mascot_image_url: Option<String>,
  #[sea_orm(column_name = "bannerUrl")]
  pub banner_url: Option<String>,
  #[sea_orm(column_name = "iconUrl")]
  pub icon_url: Option<String>,
  #[sea_orm(column_name = "cacheRemoteFiles")]
  pub cache_remote_files: bool,
  #[sea_orm(column_name = "enableRecaptcha")]
  pub enable_recaptcha: bool,
  #[sea_orm(column_name = "recaptchaSiteKey")]
  pub recaptcha_site_key: Option<String>,
  #[sea_orm(column_name = "recaptchaSecretKey")]
  pub recaptcha_secret_key: Option<String>,
  #[sea_orm(column_name = "urlPreviewSummaryProxyUrl")]
  pub url_preview_summary_proxy_url: Option<String>,
  #[sea_orm(column_name = "enableEmail")]
  pub enable_email: bool,
  pub email: Option<String>,
  #[sea_orm(column_name = "smtpSecure")]
  pub smtp_secure: bool,
  #[sea_orm(column_name = "smtpHost")]
  pub smtp_host: Option<String>,
  #[sea_orm(column_name = "smtpPort")]
  pub smtp_port: Option<i32>,
  #[sea_orm(column_name = "smtpUser")]
  pub smtp_user: Option<String>,
  #[sea_orm(column_name = "smtpPass")]
  pub smtp_pass: Option<String>,
  #[sea_orm(column_name = "enableServiceWorker")]
  pub enable_service_worker: bool,
  #[sea_orm(column_name = "swPublicKey")]
  pub sw_public_key: Option<String>,
  #[sea_orm(column_name = "swPrivateKey")]
  pub sw_private_key: Option<String>,
  #[sea_orm(column_name = "pinnedUsers")]
  pub pinned_users: Vec<String>,
  #[sea_orm(column_name = "termsOfServiceUrl")]
  pub terms_of_service_url: Option<String>,
  #[sea_orm(column_name = "repositoryUrl")]
  pub repository_url: Option<String>,
  #[sea_orm(column_name = "feedbackUrl")]
  pub feedback_url: Option<String>,
  #[sea_orm(column_name = "useObjectStorage")]
  pub use_object_storage: bool,
  #[sea_orm(column_name = "objectStorageBucket")]
  pub object_storage_bucket: Option<String>,
  #[sea_orm(column_name = "objectStoragePrefix")]
  pub object_storage_prefix: Option<String>,
  #[sea_orm(column_name = "objectStorageBaseUrl")]
  pub object_storage_base_url: Option<String>,
  #[sea_orm(column_name = "objectStorageEndpoint")]
  pub object_storage_endpoint: Option<String>,
  #[sea_orm(column_name = "objectStorageRegion")]
  pub object_storage_region: Option<String>,
  #[sea_orm(column_name = "objectStorageAccessKey")]
  pub object_storage_access_key: Option<String>,
  #[sea_orm(column_name = "objectStorageSecretKey")]
  pub object_storage_secret_key: Option<String>,
  #[sea_orm(column_name = "objectStoragePort")]
  pub object_storage_port: Option<i32>,
  #[sea_orm(column_name = "objectStorageUseSSL")]
  pub object_storage_use_ssl: bool,
  #[sea_orm(column_name = "proxyAccountId")]
  pub proxy_account_id: Option<String>,
  #[sea_orm(column_name = "objectStorageUseProxy")]
  pub object_storage_use_proxy: bool,
  #[sea_orm(column_name = "enableHcaptcha")]
  pub enable_hcaptcha: bool,
  #[sea_orm(column_name = "hcaptchaSiteKey")]
  pub hcaptcha_site_key: Option<String>,
  #[sea_orm(column_name = "hcaptchaSecretKey")]
  pub hcaptcha_secret_key: Option<String>,
  #[sea_orm(column_name = "objectStorageSetPublicRead")]
  pub object_storage_set_public_read: bool,
  #[sea_orm(column_name = "backgroundImageUrl")]
  pub background_image_url: Option<String>,
  #[sea_orm(column_name = "logoImageUrl")]
  pub logo_image_url: Option<String>,
  #[sea_orm(column_name = "objectStorageS3ForcePathStyle")]
  pub object_storage_s3_force_path_style: bool,
  #[sea_orm(column_name = "deeplAuthKey")]
  pub deepl_auth_key: Option<String>,
  #[sea_orm(column_name = "deeplIsPro")]
  pub deepl_is_pro: bool,
  #[sea_orm(column_name = "emailRequiredForSignup")]
  pub email_required_for_signup: bool,
  #[sea_orm(column_name = "themeColor")]
  pub theme_color: Option<String>,
  #[sea_orm(column_name = "defaultLightTheme")]
  pub default_light_theme: Option<String>,
  #[sea_orm(column_name = "defaultDarkTheme")]
  pub default_dark_theme: Option<String>,
  #[sea_orm(column_name = "sensitiveMediaDetection")]
  pub sensitive_media_detection: MetaSensitivemediadetectionEnum,
  #[sea_orm(column_name = "sensitiveMediaDetectionSensitivity")]
  pub sensitive_media_detection_sensitivity: MetaSensitivemediadetectionsensitivityEnum,
  #[sea_orm(column_name = "setSensitiveFlagAutomatically")]
  pub set_sensitive_flag_automatically: bool,
  #[sea_orm(column_name = "enableIpLogging")]
  pub enable_ip_logging: bool,
  #[sea_orm(column_name = "enableSensitiveMediaDetectionForVideos")]
  pub enable_sensitive_media_detection_for_videos: bool,
  #[sea_orm(column_name = "enableActiveEmailValidation")]
  pub enable_active_email_validation: bool,
  #[sea_orm(column_name = "enableTurnstile")]
  pub enable_turnstile: bool,
  #[sea_orm(column_name = "turnstileSiteKey")]
  pub turnstile_site_key: Option<String>,
  #[sea_orm(column_name = "turnstileSecretKey")]
  pub turnstile_secret_key: Option<String>,
  #[sea_orm(column_type = "JsonBinary")]
  pub policies: Json,
  #[sea_orm(column_name = "sensitiveWords")]
  pub sensitive_words: Vec<String>,
  #[sea_orm(column_name = "enableChartsForRemoteUser")]
  pub enable_charts_for_remote_user: bool,
  #[sea_orm(column_name = "enableChartsForFederatedInstances")]
  pub enable_charts_for_federated_instances: bool,
  #[sea_orm(column_name = "serverRules")]
  pub server_rules: Vec<String>,
  #[sea_orm(column_name = "preservedUsernames")]
  pub preserved_usernames: Vec<String>,
  #[sea_orm(column_name = "serverErrorImageUrl")]
  pub server_error_image_url: Option<String>,
  #[sea_orm(column_name = "notFoundImageUrl")]
  pub not_found_image_url: Option<String>,
  #[sea_orm(column_name = "infoImageUrl")]
  pub info_image_url: Option<String>,
  #[sea_orm(column_name = "enableServerMachineStats")]
  pub enable_server_machine_stats: bool,
  #[sea_orm(column_name = "enableIdenticonGeneration")]
  pub enable_identicon_generation: bool,
  #[sea_orm(column_name = "cacheRemoteSensitiveFiles")]
  pub cache_remote_sensitive_files: bool,
  #[sea_orm(column_name = "app192IconUrl")]
  pub app192_icon_url: Option<String>,
  #[sea_orm(column_name = "app512IconUrl")]
  pub app512_icon_url: Option<String>,
  #[sea_orm(column_name = "manifestJsonOverride")]
  pub manifest_json_override: String,
  #[sea_orm(column_name = "shortName")]
  pub short_name: Option<String>,
  #[sea_orm(column_name = "impressumUrl")]
  pub impressum_url: Option<String>,
  #[sea_orm(column_name = "privacyPolicyUrl")]
  pub privacy_policy_url: Option<String>,
  #[sea_orm(column_name = "perLocalUserUserTimelineCacheMax")]
  pub per_local_user_user_timeline_cache_max: i32,
  #[sea_orm(column_name = "perRemoteUserUserTimelineCacheMax")]
  pub per_remote_user_user_timeline_cache_max: i32,
  #[sea_orm(column_name = "perUserHomeTimelineCacheMax")]
  pub per_user_home_timeline_cache_max: i32,
  #[sea_orm(column_name = "perUserListTimelineCacheMax")]
  pub per_user_list_timeline_cache_max: i32,
  #[sea_orm(column_name = "notesPerOneAd")]
  pub notes_per_one_ad: i32,
  #[sea_orm(column_name = "silencedHosts")]
  pub silenced_hosts: Vec<String>,
  #[sea_orm(column_name = "enableFanoutTimeline")]
  pub enable_fanout_timeline: bool,
  #[sea_orm(column_name = "enableFanoutTimelineDbFallback")]
  pub enable_fanout_timeline_db_fallback: bool,
  #[sea_orm(column_name = "verifymailAuthKey")]
  pub verifymail_auth_key: Option<String>,
  #[sea_orm(column_name = "enableVerifymailApi")]
  pub enable_verifymail_api: bool,
  #[sea_orm(column_name = "bannedEmailDomains")]
  pub banned_email_domains: Vec<String>,
  #[sea_orm(column_name = "truemailInstance")]
  pub truemail_instance: Option<String>,
  #[sea_orm(column_name = "truemailAuthKey")]
  pub truemail_auth_key: Option<String>,
  #[sea_orm(column_name = "enableTruemailApi")]
  pub enable_truemail_api: bool,
  #[sea_orm(column_name = "enableMcaptcha")]
  pub enable_mcaptcha: bool,
  #[sea_orm(column_name = "mcaptchaSitekey")]
  pub mcaptcha_sitekey: Option<String>,
  #[sea_orm(column_name = "mcaptchaSecretKey")]
  pub mcaptcha_secret_key: Option<String>,
  #[sea_orm(column_name = "mcaptchaInstanceUrl")]
  pub mcaptcha_instance_url: Option<String>,
  #[sea_orm(column_name = "prohibitedWords")]
  pub prohibited_words: Vec<String>,
  #[sea_orm(column_name = "urlPreviewEnabled")]
  pub url_preview_enabled: bool,
  #[sea_orm(column_name = "urlPreviewTimeout")]
  pub url_preview_timeout: i32,
  #[sea_orm(column_name = "urlPreviewMaximumContentLength")]
  pub url_preview_maximum_content_length: i64,
  #[sea_orm(column_name = "urlPreviewRequireContentLength")]
  pub url_preview_require_content_length: bool,
  #[sea_orm(column_name = "urlPreviewUserAgent")]
  pub url_preview_user_agent: Option<String>,
  #[sea_orm(column_name = "mediaSilencedHosts")]
  pub media_silenced_hosts: Vec<String>,
  #[sea_orm(column_name = "inquiryUrl")]
  pub inquiry_url: Option<String>,
  #[sea_orm(column_name = "enableReactionsBuffering")]
  pub enable_reactions_buffering: bool,
  #[sea_orm(column_name = "enableStatsForFederatedInstances")]
  pub enable_stats_for_federated_instances: bool,
  pub federation: String,
  #[sea_orm(column_name = "federationHosts")]
  pub federation_hosts: Vec<String>,
  #[sea_orm(column_name = "enableTestcaptcha")]
  pub enable_testcaptcha: bool,
  #[sea_orm(column_name = "prohibitedWordsForNameOfUser")]
  pub prohibited_words_for_name_of_user: Vec<String>,
  #[sea_orm(column_name = "googleAnalyticsMeasurementId")]
  pub google_analytics_measurement_id: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(
    belongs_to = "super::user::Entity",
    from = "Column::ProxyAccountId",
    to = "super::user::Column::Id",
    on_update = "NoAction",
    on_delete = "SetNull"
  )]
  User,
}

impl Related<super::user::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::User.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}
