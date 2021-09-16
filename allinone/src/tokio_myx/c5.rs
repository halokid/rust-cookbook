use std::thread::sleep;
use tokio::time::Duration;
use std::time;
use std::thread;

#[tokio::main]
pub async fn comm() {
  println!("主线程 位置1 id {:?}", thread::current().id());
  let start = time::Instant::now();

  tokio::task::spawn(async {    // todo： 每个spawn都是在抢占CPU
    println!("线程hi1 id {:?}", thread::current().id());
    sleep(Duration::from_secs(6));
    println!("hi1");
  });

  tokio::task::spawn(async {
    println!("线程hi2 id {:?}", thread::current().id());
    sleep(Duration::from_secs(4));
    println!("hi2");
  });

  tokio::task::spawn(async {  // todo: 任务先抢占到线程就先输出hi1，操作耗时多，最后输出
    println!("线程hi3 id {:?}", thread::current().id());
    println!("hi3");
  });

  tokio::task::spawn(async {  // todo: 任务先抢占到线程就先输出hi2，操作耗时多，最后输出
    println!("线程hi4 id {:?}", thread::current().id());
    println!("hi4");
  });

  // todo: 无论起多少个spawn， 都会在默认的CPU核数的线程池中运行任务
  for i in 0..800 {
    // let j = i;
    tokio::task::spawn(async move {  // todo: 任务先抢占到线程就先输出hi2，操作耗时多，最后输出
      // println!("循环spawn次数{}, 线程id {:?}", &i, thread::current().id());
      println!("循环spawn次数{}, 线程id {:?}", i, thread::current().id());
    });
  }

  println!("hello");
  sleep(Duration::from_secs(50));     // 可以延时等待， 也可以await等待
  println!("耗时 {:?}", start.elapsed());
}

