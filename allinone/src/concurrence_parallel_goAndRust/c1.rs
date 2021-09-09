
use std::time::Duration;
use futures::executor::block_on;
use async_std::task;

// todo: 这是一个单线程异步程序
pub fn comm() {
  let future = async_main();
  // todo: The block_on executor that we used blocks the main thread, which means that all the concurrency happened on a single thread.
  // todo: 按照上面这个说法， 那就是在同一个线程里面执行了异步逻辑？ 那么rust的 async/await 就有点像 go的协程了， 可以在多个线程中调度， 也可以在一个线程中调度， 只不过go的这个调度由go runtime决定， rust的这个 async/await 的调度有更大的自由度， 有一些可以自己代码来决定？
  block_on(future);
}

async fn async_main() {
  print_for_five("await 1秒").await;

  let async_one = print_for_five("并发 async 1");
  let async_two = print_for_five("并发 async 2");

  futures::join!(async_one, async_two);
}

async fn print_for_five(msg: &str) {
  for _ in 0..5 {
    task::sleep(Duration::from_secs(1)).await;
    println!("一秒过去了: {}", msg);
  }
}



