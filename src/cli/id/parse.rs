use crate::util::id::{aid, aidx, ulid};

pub fn parse(id: &str, id_type: &str) -> String {
    match id_type {
        "aid" => aid::formatted_time(id),
        "aidx" => aidx::formatted_time(id),
        "ulid" => ulid::formatted_time(id),
        _ => "Unknown or Unsupported ID type".to_string(),
    }
}
