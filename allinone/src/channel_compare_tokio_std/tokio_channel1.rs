
// todo: case for use tokio::sync::mpsc::channel;

use std::thread;
use std::thread::sleep;
use std::time::Duration;
use tokio::sync::mpsc;
use chrono::prelude::*;

#[tokio::main]
pub async fn comm() {
  let (sx, mut rx) = mpsc::channel(3);

  tokio::task::spawn(async move {
    println!("1 now time is {:?}", Local::now());
    sx.send("hello".to_string()).await.unwrap();
    sx.send("hello_1".to_string()).await.unwrap();
  });

  while let Some(s) = rx.recv().await {
    println!("2 now time is {:?}", Local::now());
    println!("=== recv from tokio spawn --- {}", s);
  }

  sleep(Duration::from_secs(10));
  println!("3 now time is {:?}", Local::now());

  println!("====================================================================");
  // loop send ----------------------------------------
  let (sx, mut rx) = mpsc::channel(1);

  tokio::task::spawn(async move {
    for i in 0..9 {
      thread::sleep(std::time::Duration::from_secs(5));
      println!("=== send by tokio spawn --- {}, {}", i, Local::now());
      sx.send(i).await.unwrap();
    }
  });

  while let Some(i) = rx.recv().await {
    // println!("5 now time is {:?}", Local::now());
    println!("=== recv from tokio spawn --- {}, {}", i, Local::now());
  }

  sleep(Duration::from_secs(10));
  println!("6 now time is {:?}", Local::now());
}




