use tokio::{net::TcpListener, sync::broadcast, time::{sleep, Duration}};

pub async fn c1() {
  let (tx, _rx) = broadcast::channel::<String>(32);

  for i in 0..3 {
    let mut rx = tx.subscribe();

    tokio::spawn(async move {
      while let Ok(msg) = rx.recv().await {
        println!("Worker {} got: {}", i, msg);
      }
    });
  }

  for n in 0..3 {
    let msg = format!("broadcast-{}", n);
    tx.send(msg).unwrap();
    sleep(Duration::from_secs(1)).await;
  }

  sleep(Duration::from_secs(3)).await;
}