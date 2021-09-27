// todo: 异步场景6： 多个异步任务抢占CPU执行, 每个task没有超时限制, 这里并没有计算每个任务的执行时间， 执行时间短的就先返回，不是这样， 而是无论task的执行时间是多少， 哪个task先抢到CPU， 程序就执行哪个task, 然后设置一个固定的 所有执行执行的总时间， 到达了这个总时间就退出并行执行task


use tokio::time::{self as tokio_time, Duration};
use std::thread::sleep;
use std::time as timex;

async fn do_task1_async() {
  println!("task1抢到CPU，task1 和 task2 一直在抢占CPU执行, 直到超时");
  // sleep(Duration::from_secs(5));
}

async fn do_task2_async() {
  println!("task2抢到CPU，task1 和 task2 一直在抢占CPU执行, 直到超时");
  // sleep(Duration::from_secs(5));
}


#[tokio::main]
pub async fn comm() {
  let start = timex::Instant::now();
  let mut sleep = tokio_time::sleep(Duration::from_secs(3));

  loop {
    tokio::select! {
            _ = &mut sleep => {
                println!("---select!超时退出---");
                break;
            }
            _ = do_task1_async() => {
                println!("11111111111111---task1执行完毕 1 次");
            }
            _ = do_task2_async() => {
                println!("22222222222222---task2执行完毕 1 次");
            }
        }
  }

  println!("耗时 {:?}", start.elapsed());
}








