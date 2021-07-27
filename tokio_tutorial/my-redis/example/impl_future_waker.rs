use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use std::thread;

struct Delay {
  when: Instant,
}

impl Future for Delay {
  type Output = &'static str;

  fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    if Instant::now() > self.when {
      println!("Hello world!");
      Poll::Ready("done")
    } else {
      // get a handle to the waker for the current task
      let waker = cx.waker().clone();
      let when = self.when;

      // spawn a timer thread
      // todo: 建立一个线程专门用于监听，做事件驱动， 符合条件之后，再回到 waker的线程 唤醒它
      thread::spawn(move || {
        let now = Instant::now();

        if now < when {
          thread::sleep(when - now);
        }

        waker.wake();     // todo: 现在，当等待的时间到达后，调用的任务会收到通知，然后执行器就能够确认该任务能够继续调度执行了。
      });

      Poll::Pending
    }
  }
}

// todo: 在 main 函数中，我们初始化了一个 Future 并调用了其 .await，在异步的函数中，我们可以在任何实现了 Future Trait 的值上调用 .await。

#[tokio::main]
async fn main() {
  // let when = Instant::now() + Duration::from_millis(10);
  let when = Instant::now() + Duration::from_secs(5);   // 等待5秒
  let future = Delay {when};

  let out = future.await;
  println!("out ------- {:?}", out);
}