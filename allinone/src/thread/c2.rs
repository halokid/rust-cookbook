
use std::thread;

pub fn comm() {
  // 创建一个线程， 名称为 thread1， 堆栈大小为4K
  let new_thread_result = thread::Builder::new()
    .name("thread1".to_string())
    .stack_size(4 * 1024 * 1024).spawn(move || {

    println!("I am thread1.");
  });

  // 等待新创建的线程执行完成
  new_thread_result.unwrap().join().unwrap();
}