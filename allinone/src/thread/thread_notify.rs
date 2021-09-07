/*
线程之间互相通知
 */

use std::sync::{Arc, Mutex, Condvar};
use std::thread;

pub fn comm() {
  let pair = Arc::new( (Mutex::new(false), Condvar::new()) );
  let pair2 = pair.clone();

  // 创建一个线程
  thread::spawn(move || {
    let &(ref lock, ref cvar) = &*pair2;
    let mut started = lock.lock().unwrap();
    *started = true;   // todo: 这里改变 started 的值，
    cvar.notify_one();    // todo: 执行了这一句之后， 主线程的 started = cvar.wait(started).unwrap(); 就会获取到 started 新的数据为 true
    println!("通知主线程");
  });

  let &(ref lock, ref cvar) = &*pair;
  // todo: 等待子线程的通知
  let mut started = lock.lock().unwrap();

  while !*started {
    println!("before wait");
    started = cvar.wait(started).unwrap();
    println!("after wait");
  }
}

