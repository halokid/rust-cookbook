
use std::thread::sleep;
use std::thread;
use tokio::time::Duration;
use std::time;

// todo: 所谓任务阻塞，就是线程只能在完成任务A之后，才去执行任务B

#[tokio::main]
pub async fn comm() {
  let start = time::Instant::now();
  println!("主线程 id {:?}", thread::current().id());

  tokio::task::spawn_blocking(|| {
    println!("线程A id {:?}", thread::current().id());
    // todo:  运行在一个阻塞的线程，可以看作是一个比较耗时的操作
    sleep(Duration::from_secs(6));
    println!("hi");
  }).await.unwrap();    // todo: await作用就是阻塞等待该task执行完成之后， 再回到主线程

  //要等待阻塞线程完成后，主线程才能执行
  println!("hello");
  println!("耗时 {:?}", start.elapsed());
}