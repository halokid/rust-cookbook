
use std::thread::sleep;
use std::thread;
use tokio::time::Duration;
use std::time;
use tokio::runtime::Builder;

// todo: 异步场景4： 多个异步非阻塞任务串行执行, 多个任务之间不必要进行数据交互, 主线程需要等待多个任务执行完毕
#[tokio::main]      // todo: tokio::main装饰的fn 必须是 async
pub async fn comm() {
  let start = time::Instant::now();
  // todo: 不是在 tokio::main 装饰下的fn ， 是不能直接调用 tokio::task 的
  tokio::task::spawn(async {    // todo: spawn就是异步非阻塞, spawn_blocking 就是异步阻塞
    sleep(Duration::from_secs(5));
    println!("任务 1 完成");
  }).await;

  // todo: 任务1 有await关键字， 等待 1 完成了 2， 3会同时执行
  tokio::task::spawn(async {    // todo: spawn就是异步非阻塞, spawn_blocking 就是异步阻塞
    sleep(Duration::from_secs(3));
    println!("任务 2 完成");
  });

  tokio::task::spawn(async {    // todo: spawn就是异步非阻塞, spawn_blocking 就是异步阻塞
    sleep(Duration::from_secs(4));
    println!("任务 3 完成");
  }).await;

  println!("---await关键字让主线程需要等待任务执行---");
  println!("耗时 {:?}", start.elapsed());
}





