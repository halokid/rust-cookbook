use std::thread::sleep;
use tokio::time::Duration;
use std::time;

#[tokio::main]
pub async fn comm() {
  let start = time::Instant::now();

  tokio::task::spawn(async {
    println!("hi1");      // 一定是最先输出
  }).await;

  tokio::task::spawn(async {
    println!("hi2");    // 任务所属线程可能抢占不到执行所需的资源， 这里有可能还没执行， 主线程直接退出了
  });

  println!("hello");      // 切到主线程， 第二输出
  // println!("耗时 {:?}", start.elapsed());
}

