use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

struct Delay {
  when: Instant,
}

enum MainFuture {
  State0,
  State1(Delay),
  Terminated,
}

impl Future for MainFuture {
  type Output = ();

  fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    use MainFuture::*;

    loop {
      match *self {
        State0 => {
          let when = Instant::now() + Duration::from_millis(10);
          let future = Delay { when };
          *self = State1()
        }
        State(_) => {}
        Terminated => {}
      }
    }
  }
}

fn main() {}