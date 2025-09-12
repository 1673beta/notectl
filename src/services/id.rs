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
  ParseError(#[from] ParseError),
}

#[derive(Debug, Clone, Error)]
pub enum ParseError {
  ParseIntError(#[from] std::num::ParseIntError),
  UlidDecodeError(#[from] ulid::DecodeError),
}

impl std::fmt::Display for ParseError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ParseError::ParseIntError(e) => write!(f, "ParseIntError: {}", e),
      ParseError::UlidDecodeError(e) => write!(f, "UlidDecodeError: {}", e),
    }
  }
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
    let result: Result<std::time::SystemTime, ParseError> = match self.method {
      IdMethod::Aid => aid::parse(id).map_err(|e| ParseError::ParseIntError(e)),
      IdMethod::Aidx => aidx::parse(id).map_err(|e| ParseError::ParseIntError(e)),
      IdMethod::Meid => meid::parse(id).map_err(|e| ParseError::ParseIntError(e)),
      IdMethod::ObjectId => objectid::parse(id).map_err(|e| ParseError::ParseIntError(e)),
      IdMethod::Ulid => ulid::parse(id).map_err(|e| ParseError::UlidDecodeError(e)),
    };

    result.map_err(|e| IdServiceError::ParseError(e))
  }

  // TODO: validation
}
