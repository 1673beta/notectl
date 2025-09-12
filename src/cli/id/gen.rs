use crate::configs::server::IdMethod;
use crate::services::id::IdService;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn gen(format: IdMethod) -> String {
  let time = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_millis() as u64;
  let id_service = IdService::new(format);
  let result = id_service.gen(time).unwrap();
  result
}
