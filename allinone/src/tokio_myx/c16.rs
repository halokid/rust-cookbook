// todo: 异步场景7： 多个异步非阻塞任务并行执行, 每个任务执行完汇总返回数据给主线程的变量

/*
use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let (tx1, mut rx1) = oneshot::channel();
    let (tx2, mut rx2) = oneshot::channel();

    tokio::spawn(async move {
        tx1.send("first").unwrap();
    });

    tokio::spawn(async move {
        tx2.send("second").unwrap();
    });

    let mut a = None;
    let mut b = None;

    while a.is_none() || b.is_none() {
        tokio::select! {
            v1 = (&mut rx1), if a.is_none() => a = Some(v1.unwrap()),
            v2 = (&mut rx2), if b.is_none() => b = Some(v2.unwrap()),
        }
    }

    let res = (a.unwrap(), b.unwrap());

    assert_eq!(res.0, "first");
    assert_eq!(res.1, "second");
}
 */

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




