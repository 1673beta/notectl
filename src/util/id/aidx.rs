use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::num::ParseIntError;

use chrono::{DateTime, Local};

const TIME2000: u64 = 946684800000;

pub fn parse(id: &str) -> Result<SystemTime, ParseIntError> {
    let time_part = &id[0..8];
    let time = u64::from_str_radix(time_part, 36)? + TIME2000;
    Ok(UNIX_EPOCH + Duration::from_millis(time))
}

pub fn formatted_time(id: &str) -> String {
    let systime = parse(id).unwrap();
    let datetime: DateTime<Local> = systime.into();
    datetime.to_rfc3339()
}