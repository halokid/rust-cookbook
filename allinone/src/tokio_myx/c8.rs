use std::thread::sleep;
use tokio::time::Duration;
use std::time;
use tokio::runtime::{Builder};
use std::thread;
// extern crate rand;
use rand::Rng;

// todo: 单线程环境下任务阻塞和任务不阻塞示例

pub fn comm() {
  println!("主线程 位置1 id {:?}", thread::current().id());
  let start = time::Instant::now();

  //构造单线程tokio运行环境
  // todo: 建立 tokio runtime 的时候， 不用async函数本身
  // todo: 构建一个用户态运行时（也就是用户态线程，类似于协程）, 协程的最大并行数也是不超过CPU最大核数的，因为要超过根本就不是并行了， 而是并发了。。
  let runtime = Builder::new_multi_thread().
    max_threads(12).   // todo: 最多只能生成CPU核数那么多的线程数, 最多同时只能有CPU核数一样的线程并发（也就是并行）
    // max_threads(3).   // todo: 大于 0 就可以并发
    enable_all().
    build().
    expect("create tokio runtime failed");

  runtime.spawn(async {//相当于tokio::task::spawn
    println!("线程hi1 id {:?}", thread::current().id());
    println!("hi1");
  });

  runtime.spawn(async {//相当于tokio::task::spawn
    println!("线程hi2 id {:?}", thread::current().id());
    println!("hi2");//处于单线程中
  });

  // let mut rng = rand::thread_rng();
  for i in 0..20000 {
    // let j = i;
    runtime.spawn(async move {
      let mut rng = rand::thread_rng();
      let rand_n: u64 = rng.gen_range(0, 10);
      sleep(Duration::from_secs(rand_n));   // todo: 交叉阻塞sleep， 做多也不会超多最多CPU核数
      // println!("循环spawn次数{}, 线程id {:?}", &i, thread::current().id());
      println!("sleep了{}秒,循环spawn次数{}, 线程id {:?}", rand_n, i, thread::current().id());
    });
  }

  println!("hello");
  sleep(Duration::from_secs(20));
  println!("耗时 {:?}", start.elapsed());
}



