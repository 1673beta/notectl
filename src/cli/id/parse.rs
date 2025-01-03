use crate::util::id::{aid, aidx, meid, objectid, ulid};

pub fn parse(id: &str, id_type: &str) -> String {
    match id_type {
        "aid" => aid::formatted_time(id),
        "aidx" => aidx::formatted_time(id),
        "meid" => meid::parse_meid_with_format(id).to_string(),
        "objectid" => objectid::parse_object_id_with_format(id).to_string(),
        "ulid" => ulid::formatted_time(id),
        _ => "Unknown or Unsupported ID type".to_string(),
    }
}
