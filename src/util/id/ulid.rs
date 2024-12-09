use std::{time::{Duration, SystemTime, UNIX_EPOCH}};
use ulid::{Ulid, DecodeError};

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