use std::thread::sleep;
use tokio::time::Duration;
use std::time;
use std::thread;

#[tokio::main]
pub async fn comm() {
  println!("主线程 位置1 id {:?}", thread::current().id());
  let start = time::Instant::now();

  tokio::task::spawn(async {
    sleep(Duration::from_secs(5));
    println!("hi1");      // 一定是最先输出
  }).await;

  tokio::task::spawn(async {
    println!("hi2");    // 任务所属线程可能抢占不到执行所需的资源， 这里有可能还没执行， 主线程直接退出了
  });

  // todo: 无论起多少个spawn， 都会在默认的CPU最大核数的线程池中运行任务
  for i in 0..800 {
    // let j = i;
    tokio::task::spawn(async move {  // todo: 任务先抢占到线程就先输出hi2，操作耗时多，最后输出
      // println!("循环spawn次数{}, 线程id {:?}", &i, thread::current().id());
      println!("循环spawn次数{}, 线程id {:?}", i, thread::current().id());
    });
  }

  println!("hello");      // 切到主线程， 第二输出
  println!("耗时 {:?}", start.elapsed());
}

