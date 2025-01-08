use std::num::ParseIntError;
use std::sync::atomic::{AtomicU16, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use chrono::{DateTime, Local};
use rand::Rng;
use regex::Regex;
use lazy_static::lazy_static;

use crate::util::radix::radix_encode;

const TIME2000: u64 = 946684800000;
static COUNTER: AtomicU16 = AtomicU16::new(0);

lazy_static! {
    pub static ref AID_REGEX: Regex = Regex::new(r"^[0-9a-z](10)$").unwrap();
}

#[allow(unused)]
fn init_counter() -> u16 {
    let mut rng = rand::thread_rng();
    rng.gen::<u16>()
}

fn get_time(time: u64) -> String {
    let timestamp = std::cmp::max(0 , time - TIME2000);
    format!("{:0>8}", radix_encode(timestamp as i64, 36))
}

fn get_noise() -> String {
    let counter_val = COUNTER.fetch_add(1, Ordering::SeqCst);
    format!("{:0>2}", radix_encode(counter_val as i64, 36))
}

pub fn gen_aid(time: u64) -> Result<String, &'static str> {
    Ok(format!("{}{}", get_time(time), get_noise()))
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
