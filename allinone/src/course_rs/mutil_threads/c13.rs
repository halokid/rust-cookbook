// todo: 多线程中使用 Mutex

use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn comm() {
  // todo: 在多线程中， 无法运行的Rc<T>, Rc<T>无法在线程中传输，因为它没有实现Send特征(在下一节将详细介绍)，而该特征可以确保数据在线程中安全的传输。
  /*
  let counter = Rc::new(Mutex::new(0));
  let mut handles = vec![];

  for _ in 0..10 {
    let counter = Rc::clone(&counter);

    let handle = thread::spawn(move || {
      let mut num = counter.lock().unwrap();
      *num += 1;
    });
    handles.push(handle);
  }

  for handle in handles {
    handle.join().unwrap();
  }

  println!("Result: {}", *counter.lock().unwrap());
   */

  // todo: 多线程安全的 Arc<T>
  let counter = Arc::new(Mutex::new(0));
  let mut handles = vec![];

  for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
      let mut num = counter.lock().unwrap();
      *num += 1;
    });
    handles.push(handle);
  }

  for handle in handles {
    handle.join().unwrap();
  }

  println!("Result: {}", *counter.lock().unwrap());
}

/*
内部可变性
在之前章节，我们提到过内部可变性，其中Rc<T>和RefCell<T>的结合，可以实现单线程的内部可变性。
现在我们又有了新的武器，由于Mutex<T>可以支持修改内部数据，当结合Arc<T>一起使用时，可以实现多线程的内部可变性。
简单总结下：Rc<T>/RefCell<T>用于单线程内部可变性， Arc<T>/Mutex<T>用于多线程内部可变性
 */





