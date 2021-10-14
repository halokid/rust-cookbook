mod channel_sample;
mod io_sample;
mod async_future;
mod impl_future_waker;
mod select_sample;
mod myselect;
mod monifiying_branch;
mod streams;
mod mytest;
mod tokio_no_async;
mod tokio_async;
mod echo_client;
mod echo_server;
mod echo_client_stream;
mod pure_server;
mod pure_client;

use mini_redis::{client, Result};

#[tokio::main]
pub async fn main() -> Result<()> {
  // Open a connection to the mini-redis address.
  let mut client = client::connect("127.0.0.1:6379").await?;

  // Set the key "hello" with value "world"
  client.set("hello", "world".into()).await?;

  // Get key "hello"
  let result = client.get("hello").await?;

  println!("got value from the server; result={:?}", result);


  // ----------------------------------------------------
  /*
  // todo: 调用 `say_world()`， 但是并不执行函数里面的逻辑实体
  let op = say_world();
  println!("hello");
  // todo: 在 op 对象上调用 `.await`, 开始执行 `say_world()`
  op.await;
  */

  Ok(())
}

async fn say_world() {
  println!("world");
}

// todo: 一个异步函数
#[tokio::main]
async fn a_sync_fn() {
  println!("i am async fn");
}

// todo: 上面的异步函数的本质
fn a_sync_fn_real() {
  // 获取一个线程 runtime
  let mut rt = tokio::runtime::Runtime::new().unwrap();
  rt.block_on(async {
    println!("i am async fn");
  })
}




