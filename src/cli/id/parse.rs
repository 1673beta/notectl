use crate::configs::server::IdMethod;
use crate::services::id::IdService;

pub fn parse(id: &str, method: IdMethod) -> String {
  let id_service = IdService::new(method);
  let result = id_service.parse(id).unwrap();
  format!("{:?}", result)
}
