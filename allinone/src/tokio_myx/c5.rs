use std::thread::sleep;
use tokio::time::Duration;
use std::time;

#[tokio::main]
pub async fn comm() {
  let start = time::Instant::now();

  tokio::task::spawn(async {
    sleep(Duration::from_secs(6));
    println!("hi1");
  });

  tokio::task::spawn(async {
    sleep(Duration::from_secs(4));
    println!("hi2");
  });

  tokio::task::spawn(async {  // todo: 任务先抢占到线程就先输出hi1，操作耗时多，最后输出
    println!("hi3");
  });

  tokio::task::spawn(async {  // todo: 任务先抢占到线程就先输出hi2，操作耗时多，最后输出
    println!("hi4");
  });

  println!("hello");
  println!("耗时 {:?}", start.elapsed());
}

