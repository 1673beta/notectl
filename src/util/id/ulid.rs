use std::time::{Duration, SystemTime, UNIX_EPOCH};
use ulid::{DecodeError, Generator, Ulid};

use chrono::{DateTime, Local};

pub fn parse(id: &str) -> Result<SystemTime, DecodeError> {
  let ulid = Ulid::from_string(id)?;
  let timestamp = ulid.timestamp_ms();
  Ok(UNIX_EPOCH + Duration::from_millis(timestamp))
}

pub fn formatted_time(id: &str) -> String {
  let systime = parse(id).unwrap();
  let datetime: DateTime<Local> = systime.into();
  datetime.to_rfc3339()
}

#[allow(clippy::let_and_return)]
pub fn gen_ulid(time: u64) -> String {
  let mut gen = Generator::new();
  // ここのSystemTime::now()を引数のi64から取得するようにする
  let now = SystemTime::UNIX_EPOCH + Duration::from_millis(time);
  let ulid = gen.generate_from_datetime(now).unwrap().to_string();
  ulid
}
