

use std::thread::sleep;
use tokio::time::Duration;
use std::time;

#[tokio::main]
pub async fn comm() {
  let start = time::Instant::now();

  // delay_in_async();   // 不会执行

  tokio::task::spawn_blocking(|| {
    sleep(Duration::from_secs(6));

    println!("hi");
  }).await.unwrap();

  delay_in_async().await;    // 阻塞主线程执行

  println!("hello");
  println!("耗时{:?}", start.elapsed());
}


async fn delay_in_async() {
  tokio::time::delay_for(tokio::time::Duration::from_secs(4)).await;
  println!("在async的fn里sleep 4 秒");
}