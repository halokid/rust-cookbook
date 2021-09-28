
use std::thread::sleep;
use std::thread;
use tokio::time::Duration;
use std::time;
use tokio::sync::mpsc;
use std::option::Option::Some;
use std::sync::atomic::{AtomicUsize, Ordering, AtomicU8};
use std::sync::Arc;
use futures::TryFutureExt;

// todo: 参考 https://users.rust-lang.org/t/how-to-mutate-a-global-variable-using-tokio-and-futures/50276/4

// todo: 异步场景1： 等待一个异步阻塞操作返回，然后主线程获取数据（主线程是一个async函数，异步scope可能是一个函数， 也可能是一个spawn） 实现这个有两种方式， 1. 通过线程间通信去传递数据，  2.  使用共享内存数据来读写

#[tokio::main]
pub async fn comm() {
  // todo: +++++++ 只传数据进spawn，不用spawn返回， 传的数据会丢失生命周期，不能返回主线程+++++++++++++++
  let name = "foo".into();
  let mut say_hello = "".into();

  tokio::task::spawn_blocking(move || {
    say_hello = get_ret(name);
    println!("在spawn里面say hello ------- {}", say_hello);
    // let i = "".into();      // todo: 在async的闭包里， 外面不能获取i
  });

  // println!("say hello ------- {}", say_hello.clone());    // todo: 跨线程/协程肯定会转移变量生命首期的， 这里编译错误 ^^^^^ value borrowed here after move
  // println!("say hello i ------- {}", i);      // todo: 错误 ^ not found in this scope

  // todo: +++++++ 通过通信方式在线程之间传输数据 +++++++++++++++++++++++++++++++++++++++++++++++++
  let (tx, mut rx) = mpsc::channel(9);
  let namex = "bar".into();
  // tokio::task::spawn_blocking( move || {    // todo: spawn_blocking 是阻塞异步， 是不能调用async闭包的， 所以这里不能用 async move
  // tokio::task::spawn_blocking(async move {
  tokio::task::spawn(async move {
    let say_hello = get_ret(namex);
    tx.send(say_hello).await.unwrap();
    // tx.send(say_hello).await;
  });

  while let Some(message) = rx.recv().await {
    println!("await方式在主线程获取跨线程 say_hello通过mpsc传回主线程: {:?}", message);
  }

  // todo: 这样是不行的， 必须在其他线程 recv, 不能在主线程
  // while let Some(msg) = rx.blocking_recv() {
  //   println!("block方式在主线程获取跨线程 say_hello通过mpsc传回主线程: {:?}", msg);
  // }


  // todo: +++++++ 通过共享内存的方式 +++++++++++++++++++++++++++++++++++++++++++++++++
  let val = Arc::new(AtomicUsize::new(1));
  for _ in 0..10 {
    // println!("--- forecho oper Arc ---");
    let valx = Arc::clone(&val);

    tokio::task::spawn(async move {
      let v = valx.fetch_add(1, Ordering::SeqCst);
      println!("在spawn async中的Arc标识的共享内存: {:?}", v);
    });
  }


  // todo: +++++++ 通过共享内存的方式返回字符串, 不成功 +++++++++++++++++++++++++++++++++++++++++++++++++
  // let mut valx = Arc::new("hello");
  // let valy = Arc::clone(&valx);
  // tokio::task::spawn(async move {
  //   valx = Arc::from("xxx");
  // });
  // println!("valx ------------- {}", valx);
}

fn get_ret(name: String) -> String {
  return format!("hello {}", name)
}


