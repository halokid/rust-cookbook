use std::fmt::Alignment::Left;
use bytes::Bytes;
use mini_redis::client;
use tokio::sync::{mpsc, oneshot};

#[derive(Debug)]
enum Command {
  Get {
    key: String,
    resp: Responder<Option<Bytes>>,
  },
  Set {
    key: String,
    val: Bytes,
    resp: Responder<()>,
  },
}

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[tokio::main]
async fn main() {

  let (tx, mut rx) = mpsc::channel(32);
  let tx2 = tx.clone();

  let manager = tokio::spawn(async move {
    let mut clientx = client::connect("127.0.0.1:6379").await.unwrap();

    while let Some(cmd) = rx.recv().await {
      match cmd {
        Command::Get {key, resp} => {
          let res = clientx.get(&key).await;
          let _ = resp.send(res);
        },

        Command::Set {key, val, resp} => {
          let res = clientx.set(&key, val).await;
          let _ = resp.send(res);
        }
      }
    }
  });

  let t1 = tokio::spawn(async move {
    let (resp_tx, resp_rx) = oneshot::channel();
    let cmd = Command::Get {
      key: "foo".to_string(),
      resp: resp_tx,
    };

    // send SET command
    if tx.send(cmd).await.is_err() {
      eprint!("connection task shutdown");
      return;
    }

    // Await the response
    let res = resp_rx.await;
    println!("GOT (Set) = {:?}", res);
  });

  let t2 = tokio::spawn(async move {
    let (resp_tx, resp_rx) = oneshot::channel();
    let cmd = Command::Set {
      key: "foo2".to_string(),
      val: "bar".into(),
      resp: resp_tx,
    };

    if tx2.send(cmd).await.is_err() {
      eprint!("connection task shutdown");
      return;
    }

    let res = resp_rx.await;
    println!("GOT (Set) = {:?}", res);
  });

  t1.await.unwrap();
  t2.await.unwrap();
  manager.await.unwrap();

  /*    TODO: wrong sample
  let mut clientx = client::connect("127.0.0.1:6379")
    .await.unwrap();

  let t1 = tokio::spawn(async {
    let res = clientx.get("hello").await;
  });

  let t2 = tokio::spawn(async {
    clientx.set("foo", "bar".into()).await;
  });

  t1.await.unwrap();
  t2.await.unwrap();
   */
}




