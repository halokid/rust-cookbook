use tokio::sync::oneshot;
use tokio::time::{sleep, Duration, timeout};

pub async fn c1() {
  let mut receivers = Vec::new();

  for i in 0..5 {
    let (tx, rx) = oneshot::channel();
    receivers.push((i, rx));

    // 每个任务的工作时长递增
    tokio::spawn(async move {
      println!("🛠️ Task {} started", i);
      sleep(Duration::from_secs(i + 1)).await;
      println!("✅ Task {} done", i);
      let _ = tx.send(format!("Task {} finished", i));
    });
  }

  // 主任务设置统一超时时间（例如：最多等 3 秒）
  let timeout_limit = Duration::from_secs(3);

  for (i, rx) in receivers {
    match timeout(timeout_limit, rx).await {
      Ok(Ok(msg)) => println!("📩 Main received: {}", msg),
      Ok(Err(_)) => println!("❌ Task {}: sender dropped before sending", i),
      Err(_) => println!("⏰ Task {}: timed out after {:?}", i, timeout_limit),
    }
  }

  println!("🎉 Main: done waiting for all tasks!");
}


