
trait SimpleFuture {
  type Output;
  fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

enum Poll<T> {
  Ready(T),
  Pending,
}

pub struct Join<FutureA, FutureB> {
  a: Option<FutureA>,
  b: Option<FutureB>,
}

impl<FutureA, FutureB> SimpleFuture for Join<FutureA, FutureB>
where
  FutureA: SimpleFuture<Output = ()>,
  FutureB: SimpleFuture<Output = ()>,
{
  type Output = ();

  fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
    if let Some(a) = &mut self.a {
      if let Poll::Ready(()) = a.poll(wake) {
        self.a.take();
      }
    }

    if let Some(b) = &mut self.b {
      if let Poll::Ready(()) = b.poll(wake) {
        self.b.take();
      }
    }

    if self.a.is_none() && self.b.is_none() {
      Poll::Ready(())
    } else {
      Poll::Pending
    }
  }
}

