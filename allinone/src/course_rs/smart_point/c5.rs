use std::rc::Rc;
use std::sync::Arc;
use std::thread;

pub fn comm() {
  // todo: Rc can not use in multi threads
  // let s = Rc::new(String::from("multi thread walker"));
  let s = Arc::new(String::from("multi thread walker"));
  for _ in 0..10 {
    // let s = Rc::clone(&s);
    let s = Arc::clone(&s);
    // 首先通过 thread::spawn 创建一个线程，然后使用 move 关键字把克隆出的 s 的所有权转移到线程中
    let handle = thread::spawn(move || {
      println!("-->>> {}", s);
    });
  }
}