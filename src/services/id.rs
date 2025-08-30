use crate::util::id::*;
use thiserror::Error;
use crate::configs::server::IdMethod;

#[derive(Error, Debug)]
pub enum IdServiceError {
  #[error("Invalid ID format")]
  InvalidIdFormat,

  #[error("ID generation error. {reason} ")]
  IdGenerationError { reason: String },

  #[error("Unsafe ID detected")]
  UnsafeIdError,

  #[error("Parsing error: {0}")]
  ParseError(#[from] std::num::ParseIntError),
}

pub struct IdService {
  pub method: IdMethod,
}

impl IdService {
  pub fn new(method: IdMethod) -> Self {
    Self { method }
  }

  pub fn gen(&self, time: u64) -> Result<String, IdServiceError> {
    let result = match self.method {
      IdMethod::Aid => aid::gen_aid(time),
      IdMethod::Aidx => aidx::gen_aidx(time),
      IdMethod::Meid => meid::gen_meid(time),
      IdMethod::ObjectId => objectid::gen_object_id(time),
      IdMethod::Ulid => ulid::gen_ulid(time),
    };

    result.map_err(|reason| IdServiceError::IdGenerationError {
      reason: reason.to_string(),
    })
  }

  pub fn parse(&self, id: &str) -> Result<std::time::SystemTime, IdServiceError> {
    let result = match self.method {
      IdMethod::Aid => aid::parse(id),
      IdMethod::Aidx => aidx::parse(id),
      IdMethod::Meid => meid::parse(id),
      IdMethod::ObjectId => objectid::parse(id),
      IdMethod::Ulid => ulid::parse(id),
    };

    result.map_err(|e| IdServiceError::ParseError(e))
  }

  // TODO: validation
}
