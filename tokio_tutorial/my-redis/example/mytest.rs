
use futures::executor::block_on;
use std::{thread, time};
use std::time::Duration;
use tokio::runtime::Runtime;
use std::error::Error;

async fn concurr_do() {
  // thread::sleep(time::Duration::from_secs(3));
  tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
  println!("concurrence doing 2");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  tokio::spawn( async {
    // todo: 这种是会在主线程等待的
    // thread::sleep(time::Duration::from_secs(5));

    // todo: 这种是不会在主线程等待的
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    // tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    println!("concurrence doing 1");
  // });      // todo: 这样的话，线程还是会执行，但是不会阻塞主线程，也就是主线程不用等待
  }).await;   // todo: 这样的话，这里就会等待这个线程的执行

  // concurr_do().await;
  let act = concurr_do();     // todo: 不会执行
  act.await;    // todo: 等待线程执行

  println!("DONE");

  Ok(())
}