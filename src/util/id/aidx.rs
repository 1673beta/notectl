use std::num::ParseIntError;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use chrono::{DateTime, Local};
use lazy_static::lazy_static;
use nanoid::nanoid;
use regex::Regex;

use crate::util::radix::radix_encode;

const TIME2000: u64 = 946684800000;
// const TIME_LENGTH: usize = 8;
const NODE_LENGTH: usize = 4;
const NOISE_LENGTH: usize = 4;

static COUNTER: AtomicU32 = AtomicU32::new(0);
lazy_static! {
  pub static ref AIDX_REGEX: Regex = Regex::new(r"^[0-9a-z]{16}$").unwrap();
  pub static ref NODE_ID: String = nanoid!(
    NODE_LENGTH,
    &[
      '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
      'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
    ]
  );
}

fn get_time(time: u64) -> String {
  let time = time as i64 - TIME2000 as i64;
  let timestamp = std::cmp::max(0, time);
  format!("{:0>8}", radix_encode(timestamp as i64, 36))
}

fn get_noise() -> String {
  let counter = COUNTER.fetch_add(1, Ordering::SeqCst);
  format!(
    "{:0>width$}",
    format!("{:x}", counter),
    width = NOISE_LENGTH
  )
}

pub fn gen_aidx(time: u64) -> Result<String, &'static str> {
  Ok(format!("{}{}{}", get_time(time), &*NODE_ID, get_noise()))
}

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
