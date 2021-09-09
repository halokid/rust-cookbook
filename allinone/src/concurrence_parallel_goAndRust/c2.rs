
use std::thread;
use std::time::Duration;

pub fn comm() {
  thread::spawn(|| {
    for _ in 0..10 {
      println!("spawn线程等待1秒");
      thread::sleep(Duration::from_millis(1000));
    }
  });

  for _ in 0..5 {
    println!("主线程等待1秒");
    thread::sleep(Duration::from_millis(1000));
  }
}