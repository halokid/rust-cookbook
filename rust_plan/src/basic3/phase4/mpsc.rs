use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

pub async fn c1() {
  let (tx, mut rx) = mpsc::channel::<String>(32);

  // start a async manager for recv msg
  tokio::spawn(async move {
    while let Some(msg) = rx.recv().await {
      println!("Ma get msg: {}", msg);
    }
  });

  // main task create multi sub task,  send msg
  for i in 0..3 {
    let tx = tx.clone();
    tokio::spawn(async move {
      let msg = format!("worker-{} say: hello", i);
      tx.send(msg).await.unwrap();
    });
  }

  sleep(Duration::from_secs(3)).await;
}