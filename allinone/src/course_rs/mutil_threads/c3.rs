use std::sync::{Arc, Barrier};
use std::thread;

// todo: 在 Rust 中，可以使用 Barrier 让多个线程都执行到某个点后，才继续一起往后执行
pub fn comm() {
  let mut handles = Vec::with_capacity(6);
  let barrier = Arc::new(Barrier::new(6));

  for _ in 0..6 {
    let b = barrier.clone();
    handles.push(thread::spawn(move || {
      println!("before wait");
      b.wait();
      println!("after wait");
    }));
  }

  for handle in handles {
    handle.join().unwrap();
  }
}

