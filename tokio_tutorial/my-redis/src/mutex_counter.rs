
use std::sync::Mutex;

struct CanIncrement {
  mutex:  Mutex<i32>,
}

impl CanIncrement {
  // todo: 这个函数不用标记为async
  fn increment(&self) {
    let mut lock = self.mutex.lock().unwrap();
    *lock += 1;
  }
}

async fn increment_and_do_staff(can_incr: &CanIncrement) {
  // todo: 这种方式保证了不会触发 Send 错误，因为 MutexGuard 并没有出现在异步函数中。
  can_incr.increment();
  // do some thing
  do_something_async().await;
  test();
}

async fn do_something_async() {
  println!("do something async");
}

fn test() {
  println!("test");
}