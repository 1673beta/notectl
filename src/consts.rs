use serde::{Deserialize, Serialize};
use std::time::SystemTime;

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserIdentifier {
  pub username: String,
  pub host: String,
}

impl std::str::FromStr for UserIdentifier {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
      let parts: Vec<&str> = s.split('@').collect();
      match parts.len() {
        2 => Ok(UserIdentifier {
          username: parts[0].to_string(),
          host: parts[1].to_string(),
        }),
        _ => Err("Invalid user identifier".to_string()),
      }
  }
}