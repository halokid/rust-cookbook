use tokio::sync::oneshot;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct MySelect {
  rx1: oneshot::Receiver<&'static str>,
  rx2: oneshot::Receiver<&'static str>,
}

impl Future for MySelect {
  type Output = ();

  fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    // todo: poll(cx) 就是往线程(上下文句柄)去发送信号,
    if let Poll::Ready(val) = Pin::new(&mut self.rx1).poll(cx) {
      println!("rx1 compelted first with {:?}", val);
      return Poll::Ready(());
    }

    if let Poll::Ready(val) = Pin::new(&mut self.rx2).poll(cx) {
      println!("rx2 compelted first with {:?}", val);
      return Poll::Ready(());
    }

    Poll::Pending
  }
}

#[tokio::main]
async fn main() {
  let (tx1, rx1) = oneshot::channel();
  let (tx2, rx2) = oneshot::channel();

  // todo: 要发送数据给rx，不然rx会一直阻塞， 相当于 go 的无缓冲channel
  tokio::spawn(async {
    let _ = tx1.send("one");
  });

  tokio::spawn(async {
    let _ = tx2.send("two");
  });

  MySelect {
    rx1: rx1,
    rx2: rx2,
  }.await;      // todo: .await 就会触发 Future::poll 方法
}




