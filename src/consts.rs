use std::time::SystemTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeiliNotes {
    pub id: String,
    pub created_at: SystemTime,
    pub user_id: String,
    pub user_host: Option<String>,
    pub channel_id: Option<String>,
    pub cw: Option<String>,
    pub text: Option<String>,
    pub tags: Vec<String>,
}
