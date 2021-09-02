
use std::time::Instant;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
  let now = Instant::now();

  let mut handles = Vec::with_capacity(10);
  for i in 0..10 {
    // todo: 要想异步执行， 必须是传入future， 这里没有把 future 变成任务
    handles.push(my_gb_task(i));
  }

  std::thread::sleep(Duration::from_millis(120));
  println!("主线程任务完成");

  let mut handles2 = handles.copy();
  // todo: 方式1, 同步阻塞
  for handle in handles {
    // todo: handle 不是 future，所以不能异步
    handle.await;
  }

  println!("++++++++++++++++++++++++++++++++++++++++++++++++");

  // todo: 方式2, 异步阻塞
  futures::future::join_all(handles2).await;

  println!("总耗时: {} ms", now.elapsed().as_millis());
}

async fn my_gb_task(i: u64) {
  let millis = 100;
  println!("任务 {} sleeping {} ms", i, millis);
  sleep(Duration::from_millis(millis)).await;
  println!("任务 {} 完成", i);
}
