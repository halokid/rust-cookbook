use tokio::sync::oneshot;
use tokio::sync::oneshot::error::RecvError;
use tokio::time::{sleep, Duration};

pub async fn c1() {
  let (tx, rx) = oneshot::channel();

  // spawn a sub task implement work
  tokio::spawn(async move {
    println!("Child working...");
    sleep(Duration::from_secs(2)).await;
    tx.send("Child done!").unwrap();      // send only one time
  });

  println!("Waiting for child...");
  match rx.await {
    Ok(msg) => {
      println!("Main recv: {}", msg);
    }
    Err(_) => {
      println!("Child dropped before sending");
    }
  }
}