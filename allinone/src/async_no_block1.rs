/*
下面的写法是类似go的sync.waitgroup写法，同时异步执行，然后统一在某一个位置等待所有异步的执行
 */

/*
use futures::executor::block_on;
use std::{thread, time};
use std::time::Duration;
use tokio::runtime::Runtime;

async fn learn_song() -> String {
  // todo: 这种延时会阻塞所有的线程
  // thread::sleep(time::Duration::from_secs(5));

  // todo: 这种才是在不同线程里面使用延时的正确姿势
  tokio::time::delay_for(tokio::time::Duration::from_secs(5)).await;

  // task::sleep(Duration::from_secs(1)).await;
  println!("learn song");
  "learn song".to_string()
}

async fn sing_song() {
  // thread::sleep(time::Duration::from_secs(4));
  tokio::time::delay_for(tokio::time::Duration::from_secs(4)).await;
  println!("sing song");
}

async fn learn_and_sing() {
  // 在唱歌之前等待学歌完成
  // await只会阻塞当前的函数线程， 不会阻塞其他的函数线程？
  // let song = learn_song().await;
  learn_song().await;
  sing_song().await;
}

// ----------------------------------------

async fn dance() {
  // thread::sleep(time::Duration::from_secs(3));
  tokio::time::delay_for(tokio::time::Duration::from_secs(3)).await;
  println!("dance");
}

async fn async_main() {
  // let f1 = learn_and_sing();
  // let f2 = dance();

  // futures::join!(f1, f2);
  // f1.await;
  // f2.await;

  // todo: 这种就是三个函数同时并行的做法
  let f1 = learn_song();
  let f2 = sing_song();
  let f3 = dance();
  futures::join!(f1, f2, f3);

}

pub fn comm() {
  let now = time::Instant::now();

  let mut runtime = Runtime::new().unwrap();
  runtime.block_on(async_main());

  // block_on(learn_and_sing());
  // block_on(async_main());

  println!("{:?}", now.elapsed());
}

 */




