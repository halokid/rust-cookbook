use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time;
use std::thread::sleep;

/*
代码流程:
main 线程首先进入 while 循环，调用 wait 方法挂起等待子线程的通知，并释放了锁 started
子线程获取到锁，并将其修改为 true，然后调用条件变量的 notify_one 方法来通知主线程继续执行
 */
pub fn comm() {
  let pair = Arc::new( (Mutex::new(false),
                      Condvar::new()) );
  let pair2 = pair.clone();

  thread::spawn(move || {
    let &(ref lock, ref cvar) = &*pair2;
    let mut started = lock.lock().unwrap();
    println!("changing started");
    *started = true;
    // todo: 这个 notify_one() 一定会等待这个spawn全部执行完， 就算下面再sleep多久都一样
    cvar.notify_one();

    thread::sleep(time::Duration::from_secs(10));
    println!("notify main thread!");
  });

  let &(ref lock, ref cvar) = &*pair;
  let mut started = lock.lock().unwrap();
  while !*started {
    started = cvar.wait(started).unwrap();
  }
  println!("started changed");
}


