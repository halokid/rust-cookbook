use std::sync::{Arc, Mutex, Condvar};
use std::thread::{spawn, sleep};
use std::time::Duration;

/*
Mutex用于解决资源安全访问的问题，但是我们还需要一个手段来解决资源访问顺序的问题。而 Rust 考虑到了这一点，为我们提供了条件变量(Condition Variables)，它经常和Mutex一起使用，可以让线程挂起，直到某个条件发生后再继续执行，其实Condvar我们在之前的多线程章节就已经见到过，现在再来看一个不同的例子：
 */
pub fn comm() {
  let flag = Arc::new(Mutex::new(false));
  let cond = Arc::new(Condvar::new());
  let cflag = flag.clone();
  let ccond = cond.clone();

  let hdl = spawn(move || {
    let mut m = { *cflag.lock().unwrap() };
    let mut counter = 0;

    while counter < 3 {
      while !m {
        m = *ccond.wait(cflag.lock().unwrap()).unwrap();
      }

      {
        m = false;
        *cflag.lock().unwrap() = false;
      }

      counter += 1;
      println!("inner counter: {}", counter);
    }
  });

  let mut counter = 0;
  loop {
    sleep(Duration::from_millis(1000));
    *flag.lock().unwrap() = true;
    counter += 1;
    if counter > 3 {
      break;
    }
    println!("outside counter: {}", counter);
    cond.notify_one();
  }

  hdl.join().unwrap();
  println!("flag -->>> {:?}", flag);
}

