use tokio::time::sleep;
use tokio::time::Duration;

async fn c1() {
  let task1 = async {
    sleep(Duration::from_secs(2)).await;
    "Task 1 completed, use 2 seconds"
  };

  let task2 = async {
    sleep(Duration::from_secs(1)).await;
    "task 2 completed. use 1 second"
  };

  tokio::select! {
    result = task1 => {
      println!("Task 1 completed with result {:?}", result);
    }

    result = task2 => {
      println!("Task 2 completed with result {:?}", result);
    }
  }
}

async fn c2() {
  let mut counter = 0;

  loop {
    tokio::select! {
      _ = sleep(Duration::from_secs(1)) => {
        println!("One second passed");
      }

      _ = sleep(Duration::from_secs(3)) => {
        println!("Three second passed");
        break;      // stop the loop, never reach here, cuz 1 second must faster than 3 seconds
      }
    }

    counter += 1;
    if counter >= 5  {
      println!("Loop complete after 5 iterations");
      break;
    }
  }
}

#[tokio::main]
async fn main() {
  c1().await;

  c2().await;
}




