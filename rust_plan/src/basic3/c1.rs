use tokio::time::{sleep, Duration};


// pub fn c1() {}

// concurrence print
pub async fn c1() {
  let handle1 = tokio::spawn(async {
    for i in 1..=5 {
      println!("Task 1 - count: {}", i);
      sleep(Duration::from_millis(500)).await;
    }
  });

  let handle2 = tokio::spawn(async {
    for i in 1..=5 {
      println!("Task 2 - count: {}", i);
      sleep(Duration::from_millis(300)).await;
    }
  });

  let _ = tokio::join!(handle1, handle2);
}


