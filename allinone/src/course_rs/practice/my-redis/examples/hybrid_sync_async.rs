use std::time::Duration;
use tokio::runtime::Builder;
use tokio::time::sleep;

// TODO: 1. use tokio runtime
fn main() {
  let runtime = Builder::new_multi_thread()
    .worker_threads(1)
    .enable_all()
    .build()
    .unwrap();

  let mut handles = Vec::with_capacity(10);
  for i in 0..10 {
    handles.push(runtime.spawn(my_bg_task(i)));
  }

  // do some time-consuming things when background task is running
  // sleep in the `main thread`, so use `std` package
  std::thread::sleep(Duration::from_millis(750));
  println!("Finished time-consuming task.");

  // wait for the background tasks finished
  for hanle in handles {
    runtime.block_on(hanle).unwrap();
  }
}

async fn my_bg_task(i: u64) {
  let millis = 1000 - 50 * i;
  println!("Task {} sleeping for {} ms", i, millis);

  // sleep in tokio runtime, so use `tokio sleep`
  sleep(Duration::from_millis(millis)).await;

  println!("Task {} stopping.", i);
}





