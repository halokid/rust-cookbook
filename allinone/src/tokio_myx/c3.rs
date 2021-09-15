

use std::thread::sleep;
use tokio::time::Duration;
use std::time;

#[tokio::main]
pub async fn comm() {
  let start = time::Instant::now();

  //主线程
  //tokio::task::spawn等价于thread::spawn的异步使用
  // todo: 多线程是助力异步的， 有多线程异步就会给力点， 没多线程的话， 异步一样可以运行
  tokio::task::spawn(async {      // todo: task::spawn 里面必须是一个future， 所以这里一定要用 async 关键字
    sleep(Duration::from_secs(6));
    println!("hi");
  }).await.unwrap();

  println!("hello");
  println!("耗时 {:?}", start.elapsed());
}

