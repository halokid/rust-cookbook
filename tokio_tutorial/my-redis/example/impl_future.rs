use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

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
      cx.waker().wake_by_ref();
      Poll::Pending     // todo: 持续轮询
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