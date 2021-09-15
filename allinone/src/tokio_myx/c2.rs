

use std::thread::sleep;
use tokio::time::Duration;
use std::time;
use std::thread;

#[tokio::main]
pub async fn comm() {
  println!("主线程 id {:?}", thread::current().id());
  let start = time::Instant::now();

  tokio::task::spawn_blocking(|| {
    println!("线程A id {:?}", thread::current().id());
    // todo: 运行在一个阻塞的线程，可以看作是一个比较耗时的操作
    sleep(Duration::from_secs(6));
    println!("hi");
  });   // todo: 此处未使用await关键字等待阻塞线程的任务完成, 但是主线程最终肯定会等待这个异步执行完

  //无需等待阻塞线程完成，主线程直接执行
  println!("hello");
  println!("耗时 {:?}", start.elapsed());
}


