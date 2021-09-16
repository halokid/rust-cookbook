use std::thread::sleep;
use tokio::time::Duration;
use std::time;
use tokio::runtime::{Builder};

// todo: 单线程环境下任务阻塞和任务不阻塞示例

pub fn comm() {
  let start = time::Instant::now();

  /*
  //构造单线程tokio运行环境
  let runtime = Builder::new_multi_thread().
    max_threads(1).
    enable_all().
    build().
    expect("create tokio runtime failed");
  runtime.spawn(async {//相当于tokio::task::spawn
    //处于单线程中
    println!("hi1");
  });
  runtime.spawn(async {//相当于tokio::task::spawn
    println!("hi2");//处于单线程中
  });
  println!("hello");
   */

  // /*
  //构造单线程tokio运行环境
  // todo: 简历 tokio runtime 的时候， 不用async函数本身
  let runtime = Builder::new_multi_thread().
    max_threads(1).
    // max_threads(3).   // todo: 大于 1 就可以并发
    enable_all().
    build().
    expect("create tokio runtime failed");
  runtime.spawn(async {//相当于tokio::task::spawn
    // todo: 处于单线程中，执行了耗时的操作，影响了其他任务的执行
    // todo: 如果 max_threads(3). 那么就可以同时并发（看CPU的核数是否满足并行）
    // sleep(Duration::from_secs(5));
    println!("hi1");
  });
  runtime.spawn(async {//相当于tokio::task::spawn
    println!("hi2");//处于单线程中
  });
  println!("hello");
   // */

  /*
  //构造单线程tokio运行环境
  let runtime = Builder::new_multi_thread().
    max_threads(1).
    enable_all().
    build().
    expect("create tokio runtime failed");
  runtime.spawn_blocking(|| {//相当于tokio::task::spawn_blocking
    // todo: 处于专属线程池中，执行耗时的操作不影响其他任务执行
    sleep(Duration::from_secs(10));
    println!("hi1");
  });
  runtime.spawn(async {//相当于tokio::task::spawn
    println!("hi2");//处于单线程中
  });
  println!("hello");
   */

  println!("耗时 {:?}", start.elapsed());
}

