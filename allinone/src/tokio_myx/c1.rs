
use std::thread::sleep;
use tokio::time::Duration;
use std::time;

#[tokio::main]
pub async fn comm() {
  let start = time::Instant::now();

  tokio::task::spawn_blocking(|| {
    // todo:  运行在一个阻塞的线程，可以看作是一个比较耗时的操作
    sleep(Duration::from_secs(6));
    println!("hi");
  }).await.unwrap();    // todo: await作用就是阻塞等待该task执行完成之后， 再回到主线程

  //要等待阻塞线程完成后，主线程才能执行
  println!("hello");
  println!("耗时 {:?}", start.elapsed());
}