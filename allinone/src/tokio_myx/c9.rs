
/*
todo: 不要迷信异步, 异步不是万能解药， 以下场景适合异步， 其他场景要慎重考虑：
todo: 1.  并发/并行执行一些操作， 这些不同线程/协程中的操作可以共享读取，甚至是改变数据， 但是我们不用等待这些操作的返回结果
 */
use std::thread::sleep;
use tokio::time::Duration;
use std::time;
use std::thread;

// #[derive(Debug, Clone)]
#[derive(Debug, Clone, Copy)]
struct A {
  // name:     String,
  name:     i32,
}

#[tokio::main]
pub async fn comm() {
  println!("跨线程(协程-用户线程)共享变量，变量可在不同线程中共享生命周期，不会发生线程1借给线程2之后， 线程1想调用，  产生value borrowed here after move的错误");

  let j = 1;
  // let mut a = A{ name: "i am a".to_string() };
  let mut a = A{ name: 9 };

  for i in 0..9 {
    tokio::spawn(async move {
      println!("传入spawn中的j: {}", j);
      println!("spawn 中的struct要impl Copy trait, 不然在线程中共享不了: {:?}", a);
      println!("在 spawn 中的 i 循环: {}", i);

      println!("改变 struct");
      a.name = 6;
    });
  }

  println!("在主线程中的a struct: {:?}", a);
  println!("在主线程中的j: {}", j);
}

