use tokio::time::sleep;
use tokio::time::Duration;
use tokio::{join, select};

//tokio::spawn 并发执行多个任务
async fn c1() {
  let handle1 = tokio::spawn(async {
    sleep(Duration::from_secs(2)).await;
    println!("Task 1 done");
  });

  let handle2 = tokio::spawn(async {
    sleep(Duration::from_secs(1)).await;
    println!("Task 2 done");
  });

  // 等待两个任务完成
  handle1.await.unwrap();
  handle2.await.unwrap();
}


// tokio::join! 等待多个任务
pub async fn c11() {
  async fn task1() {
    sleep(Duration::from_secs(1)).await;
    println!("Task 1 finished");
  }

  async fn task2() {
    sleep(Duration::from_secs(2)).await;
    println!("Task 2 finished");
  }

  join!(task1(), task2());
  println!("All done");
}


// tokio::select! 竞速模式（谁先完成谁赢）
pub async fn c12() {
  select! {
    _ = sleep(Duration::from_secs(1)) => {
      println!("Timeout 1s");
    }

    _ = sleep(Duration::from_secs(3)) => {
      println!("Timeout 3s");
    }
  }

  println!("select done");
}






