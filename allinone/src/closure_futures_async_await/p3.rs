use futures::future;
use tokio;

async fn async_hello() {
  println!("hello, async hello");
}

pub fn comm() {
  // todo: 这样是不会执行的
  async {
    async_hello().await;
  };
}

pub fn comm2() {
  let mut rt = tokio::runtime::Runtime::new().unwrap();
  rt.block_on(async_hello());

  let async_block = async {
    println!("in async_block");
  };
  rt.block_on(async_block);

  let x = 42;
  let async_capture = async {
    println!("in async_captue, x => {}", x);
  };
  rt.block_on(async_capture);

  let y = 43;
  let async_capture = future::lazy(|_| {
    println!("in async_capture2, y => {}", y);
  });
  rt.block_on(async_capture);
}