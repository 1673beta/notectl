use std::{num::ParseIntError, time::SystemTime};

use chrono::{DateTime, TimeZone, Utc};
use lazy_static::lazy_static;
use rand::Rng;
use regex::Regex;

const CHARS: &str = "0123456789abcdef";
lazy_static! {
    static ref OBJECT_ID_REGEX: Regex = Regex::new(r"^[0-9a-f]{24}$").unwrap();
}

fn get_time(time: u64) -> String {
    format!("{:08x}", time)
}

fn get_random() -> String {
    let mut rng = rand::thread_rng();
    (0..16)
        .map(|_| {
            let idx = rng.gen_range(0..CHARS.len());
            CHARS.chars().nth(idx).unwrap()
        })
        .collect()
}

// TODO: Resultにする
pub fn gen_object_id(t: u64) -> String {
    format!("{}{}", get_time(t), get_random())
}

pub fn parse_object_id(id: &str) -> Result<SystemTime, ParseIntError> {
    let timestamp = u64::from_str_radix(&id[0..8], 16).unwrap();
    let time = SystemTime::UNIX_EPOCH + std::time::Duration::from_millis(timestamp as u64 * 1000);
    Ok(time)
}

pub fn parse_object_id_with_format(id: &str) -> DateTime<Utc> {
    let time = parse_object_id(id).unwrap();

    let duration = time.duration_since(SystemTime::UNIX_EPOCH).unwrap();
    Utc.timestamp_millis_opt(duration.as_millis() as i64)
        .unwrap()
}
