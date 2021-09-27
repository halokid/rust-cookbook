use std::thread::sleep;
use std::thread;
use tokio::time::Duration;
use std::time;
use tokio::runtime::Builder;

// todo: 异步场景5： 多个异步任务抢占CPU执行, 每个task没有超时限制, 这里并没有计算每个任务的执行时间， 执行时间短的就先返回，不是这样， 而是无论task的执行时间是多少， 哪个task先抢到CPU， 程序就执行哪个task
async fn do_task1_async() {
  println!("task1 先抢到cpu， 执行...");
  sleep(Duration::from_secs(3));    // todo: 这种形式是会阻塞主线程的， 测试不出来效果
  // tokio::time::sleep(Duration::from_secs(3));   // todo: 等于是异步等待，用在tokio::spawn中可以， 这里测试不出来
}

async fn do_task2_async() {
  println!("task2 先抢到cpu， 执行...");
  sleep(Duration::from_secs(5));
  // tokio::time::sleep(Duration::from_secs(5));
}

#[tokio::main]
pub async fn comm() {
  let start = time::Instant::now();

  tokio::select! {
        _ = do_task1_async() => {
            println!("do_stuff_async() completed first")
        }
        _ = do_task2_async() => {
            println!("more_async_work() completed first")
        }
    };

  println!("---其中一个任务完成， 即返回主线程---");
  println!("耗时 {:?}", start.elapsed());
}










