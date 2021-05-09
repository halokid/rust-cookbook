
use futures::executor::block_on;
use std::{thread, time};

async fn hello_world() {
  thread::sleep(time::Duration::from_secs(5));
  println!("hello, world!");
}

pub fn comm() {
  let now = time::Instant::now();

  thread::sleep(time::Duration::from_secs(3));

  let future = hello_world();
  block_on(future);

  println!("{:?}", now.elapsed());
}



