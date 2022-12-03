use std::thread;
use std::time::Duration;

pub fn comm() {
  /*
  thread::spawn(|| {
    for i in 1..10 {
      println!("hi number {} from spawned thread!", i);
      thread::sleep(Duration::from_secs(1));
    }
  });
   */

  // todo: style to wait the sub thread, here just add one more thread
  let handle = thread::spawn(|| {
    for i in 1..10 {
      println!("hi number {} from spawned thread!", i);
      thread::sleep(Duration::from_secs(1));
    }
  });
  handle.join().unwrap();

  for i in 1..5 {
    println!("hi number {} from the main thread!", i);
    thread::sleep(Duration::from_secs(1));
  }
  // todo: 以上输出清晰的展示了线程阻塞的作用，如果你将 handle.join 放置在
  // todo: main 线程中的 for 循环后面，那就是另外一个结果：两个线程交替输出。
  // handle.join().unwrap();

  // ---------------------------------------------------------------
  let v = vec![1, 2, 3];

  let handle = thread::spawn(move || {
    println!("Here's a vector: {:?}", v);
  });

  // drop(v); // oh no!

  handle.join().unwrap();
}






