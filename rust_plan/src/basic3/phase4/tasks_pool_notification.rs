use tokio::sync::oneshot;
use tokio::sync::oneshot::error::RecvError;
use tokio::time::{sleep, Duration};

pub async fn c1() {
  let mut receivers = Vec::new();

  for i in 0..5 {
    // create oneshot channel
    let (tx, rx) = oneshot::channel();
    receivers.push(rx);

    // start sub process
    tokio::spawn(async move {
      println!("Task {} started", i);
      let delay = Duration::from_secs(i + 1);
      sleep(delay).await;

      println!("Task {} done after {:?}", i, delay);
      let _ = tx.send(format!("Task {} done", i));
    });
  }

  // main task wait all sub tasks done
  for rx in receivers {
    match rx.await {
      Ok(msg) => {
        println!("Main received: {}", msg);
      }
      Err(_) => {
        println!("A task was dropped before sending");
      }
    }
  }

  println!("All tasks done!");
}



