
use std::thread;
use std::{time};

pub fn comm() {
  // 创建新线程
  let new_thread = thread::spawn(move || {
    // println!("I am a new thread.");

    loop {
      println!("I am a new thread.");
    }
  });


  // 等待创建的新线程完成
  // new_thread.join().unwrap();

  // todo: 主线程等待创建的新线程也可以
  // thread::sleep(time::Duration::from_secs(3))
}


