

use std::thread::sleep;
use tokio::time::Duration;
use std::time;
use std::thread;

// todo: 所谓任务不阻塞，就是线程接收了任务A之后，也可以接收任务B, 然后线程根据任务的具体请况切换处理任务A 和 任务B， 意思就是任务A、B都在处理中，只是线程切换处理而已。

#[tokio::main]
pub async fn comm() {
  println!("主线程 位置1 id {:?}", thread::current().id());
  let start = time::Instant::now();

  //主线程
  //tokio::task::spawn等价于thread::spawn的异步使用
  // todo: 多线程是助力异步的， 有多线程异步就会给力点， 没多线程的话， 异步一样可以运行
  tokio::task::spawn(async {      // todo: task::spawn 里面必须是一个future， 所以这里一定要用 async 关键字
    println!("线程A id {:?}", thread::current().id());
    sleep(Duration::from_secs(6));
    println!("hi");
  }).await.unwrap();

  println!("hello");
  println!("主线程 位置2 id {:?}", thread::current().id());
  println!("耗时 {:?}", start.elapsed());
}

