
use std::time::Instant;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> std::io::Result<()> {
  println!("---方式1, 同步阻塞--- 同步执行任务，而且每个任务都是串行执行");
  let now1 = Instant::now();
  let mut handles = Vec::with_capacity(10);
  for i in 0..10 {
    // todo: 要想异步执行， 必须是传入future， 这里没有把 future 变成任务
    handles.push(my_gb_task(i));
  }

  // let mut handles2 = handles.copy();
  for handle in handles {
    // todo: handle 不是 future，所以不能异步
    handle.await;
  }
  println!("总耗时: {} ms", now1.elapsed().as_millis());

  println!("+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++");
  println!("---方式2, 异步阻塞--- 异步并发执行任务， 阻塞调用是指调用结果返回之前，当前线程会被挂起。调用线程只有在得到结果之后才会返回。");
  let now2 = Instant::now();
  let mut handles2 = Vec::with_capacity(10);
  for i in 0..10 {
    // todo: 要想异步执行， 必须是传入future， 这里没有把 future 变成任务
    handles2.push(my_gb_task(i));
  }

  futures::future::join_all(handles2).await;

  println!("总耗时: {} ms", now2.elapsed().as_millis());

  println!("+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++");
  println!("---方式3, 异步非阻塞--- 异步并发执行任务， 非阻塞调用指在不能立刻得到结果之前，该调用不会阻塞当前线程。");
  let now3 = Instant::now();
  let mut handles3 = Vec::with_capacity(10);
  for i in 0..10 {
    handles3.push(tokio::spawn(my_gb_task(i)));
  }

  std::thread::sleep(Duration::from_millis(10));
  println!("总耗时主线程: {} ms", now3.elapsed().as_millis());
  println!("----@@@@---主线程任务完成，以上三种方式主线程都会等待任务执行完成，那么什么写法主线程才不会等待任务完成呢？---@@@---");

  for handle in handles3 {
    handle.await?;
  }
  println!("总耗时: {} ms", now3.elapsed().as_millis());




  Ok(())
}

async fn my_gb_task(i: u64) {
  let millis = 100;
  println!("任务 {} sleeping {} ms", i, millis);
  sleep(Duration::from_millis(millis)).await;    // todo: 没有.await是不会执行的
  // sleep(Duration::from_millis(millis));
  println!("任务 {} 完成", i);
}
