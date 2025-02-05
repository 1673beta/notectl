use crate::util;

use super::IdType;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn gen(format: IdType) -> String {
  let time = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_millis() as u64;
  match format {
    IdType::Aid => {
      let id = util::id::aid::gen_aid(time).unwrap();
      format!("generated aid: {}", id)
    }
    IdType::Aidx => {
      let id = util::id::aidx::gen_aidx(time).unwrap();
      format!("generated aidx: {}", id)
    }
    IdType::Meid => {
      let id = util::id::meid::gen_meid(time);
      format!("generated meid: {}", id)
    }
    IdType::ObjectId => {
      let id = util::id::objectid::gen_object_id(time);
      format!("generated objectid: {}", id)
    }
    IdType::Ulid => {
      let id = util::id::ulid::gen_ulid(time);
      format!("generated ulid: {}", id)
    }
  }
}
