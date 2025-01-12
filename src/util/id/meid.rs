use std::{num::ParseIntError, time::SystemTime};

use chrono::{DateTime, TimeZone, Utc};
use lazy_static::lazy_static;
use rand::Rng;
use regex::Regex;

const CHARS: &str = "0123456789abcdef";
lazy_static! {
    static ref MEID_REGEX: Regex = Regex::new(r"^[0-9a-f](24)$").unwrap();
}

fn get_time(mut time: u64) -> String {
    if time == 0 {
        return CHARS[0..1].to_string();
    }

    time += 0x800000000000;

    format!("{:012x}", time)
}

fn get_random() -> String {
    let mut rng = rand::thread_rng();
    (0..12)
        .map(|_| {
            let idx = rng.gen_range(0..CHARS.len());
            CHARS.chars().nth(idx).unwrap()
        })
        .collect()
}

// TODO: Resultにする
pub fn gen_meid(time: u64) -> String {
    format!("{}{}", get_time(time), get_random())
}

pub fn parse_meid(id: &str) -> Result<SystemTime, ParseIntError> {
    let timestamp = u64::from_str_radix(&id[0..12], 16).unwrap() - 0x800000000000;
    let time = SystemTime::UNIX_EPOCH + std::time::Duration::from_millis(timestamp);
    Ok(time)
}

pub fn parse_meid_with_format(id: &str) -> DateTime<Utc> {
    let time = parse_meid(id).unwrap();
    let duration = time.duration_since(SystemTime::UNIX_EPOCH).unwrap();
    Utc.timestamp_millis_opt(duration.as_millis() as i64)
        .unwrap()
}

// id: 81942b1629fec5c845bb697d
// SystemTime: 1735889660414
// date: 2025-01-03T07:34:20.414Z
