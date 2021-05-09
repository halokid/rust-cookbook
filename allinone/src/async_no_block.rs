
use futures::executor::block_on;
use std::{thread, time};

async fn learn_song() -> String {
  thread::sleep(time::Duration::from_secs(5));
  println!("learn song");
  "learn song".to_string()
}

async fn sing_song() {
  thread::sleep(time::Duration::from_secs(4));
  println!("sing song");
}

async fn dance() {
  thread::sleep(time::Duration::from_secs(3));
  println!("dance");
}

pub fn comm() {
  let now = time::Instant::now();

  let song = block_on(learn_song());     // 阻塞
  block_on(sing_song());        // 阻塞
  block_on(dance());             // 阻塞

  // 会是上面三个函数的总耗时
  println!("{:?}", now.elapsed());
}




