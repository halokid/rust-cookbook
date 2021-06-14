
use std::time::{SystemTime, UNIX_EPOCH};

extern crate time;

fn timestamp2() -> i64 {
  let timespec = time::get_time();
  timespec.sec * 1000 + (timespec.nsec as f64 / 1000.0 / 1000.0) as i64
}

fn timestamp1() -> i64 {
  let start = SystemTime::now();
  let since_the_epoch = start
    .duration_since(UNIX_EPOCH)
    .expect("Time went backwards");
  let ms = since_the_epoch.as_secs() as i64 * 1000i64 + (since_the_epoch.subsec_nanos() as f64 / 1_000_000.0) as i64;
  ms
}

fn timestamp3() -> f64 {
  let timespec = time::get_time();
  // 1459440009.113178
  // let mills: f64 = timespec.sec as f64 + (timespec.nsec as f64 / 1000.0 / 1000.0 / 1000.0);
  let mills: f64 = timespec.sec as f64 + (timespec.nsec as f64 / 1000.0 / 1000.0 / 1000.0);
  mills * 1000000 as f64 * 3 as f64
}

pub fn comm() {
  let ts1 = timestamp1();
  println!("TimeStamp1: {}", ts1);
  let ts2 = timestamp2();
  println!("TimeStamp2: {}", ts2);
  let ts3 =  timestamp3();
  println!("TimeStamp3: {}", ts3.to_string());
}
