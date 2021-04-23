use futures::Future;
use futures::{ self, executor };

async fn my_funcion1() {
  println!("Hello!");
}

// todo: my_function1 等价于 下面的函数
fn my_function2() -> impl Future<Output = ()> {
  async {
    println!("Hello!");
  }
}

// todo: 通过block_on 调用异步函数
fn main1() {
  let f = my_funcion1();
  executor::block_on(f);
}

// ------------------------------------------------------------

// todo: 通过 .await 调用
async fn learn_song() {
  println!("learn song!");
}

async fn sing_song() {
  println!("Sing song!");
}

async fn dance() {
  println!("Dance!");
}

async fn learn_and_sing_song() {
  learn_song().await;
  sing_song().await;
}

async fn async_main() {
  let f1 = learn_and_sing_song();
  let f2 = dance();
  futures::join!(f1, f2);
}

fn main2() {
  executor::block_on(async_main());
  println!("Hello, World!");
}



