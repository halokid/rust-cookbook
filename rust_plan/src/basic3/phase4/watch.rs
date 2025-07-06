use tokio::sync::watch;
use tokio::time::{sleep, Duration};

pub async fn c1() {
  let (tx, rx) = watch::channel("⏳INIT...");

  // start 2 subscribe tasks
  for i in 0..2 {
    let mut rx = rx.clone();

    tokio::spawn(async move {
      loop {
        rx.changed().await.unwrap();    // wait for change
        println!("Watcher {} sees: {}", i, *rx.borrow());
      }
    });
  }

  sleep(Duration::from_secs(1)).await;
  tx.send("🟢 READY").unwrap();
  sleep(Duration::from_secs(1)).await;
  tx.send("🔴 SHUTDOWN").unwrap();
  sleep(Duration::from_secs(1)).await;
}