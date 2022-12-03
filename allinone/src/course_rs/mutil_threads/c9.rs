use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// todo: 使用多发送者
pub fn comm() {
  /*
  let (tx, rx) = mpsc::channel();
  let tx1 = tx.clone();

  thread::spawn(move || {
    tx.send(String::from("hi from raw tx")).unwrap();
  });

  thread::spawn(move || {
    tx1.send(String::from("hi from cloned tx")).unwrap();
  });

  for received in rx {
    println!("Got: {}", received);
  }

  // todo: 异步通道
  let (tx, rx) = mpsc::channel();

  let handle = thread::spawn(move || {
    println!("发送之前");
    tx.send(1).unwrap();
    println!("发送之后");
  });

  println!("睡眠之前");
  thread::sleep(Duration::from_secs(3));
  println!("睡眠之后");

  println!("receive {}", rx.recv().unwrap());
  handle.join().unwrap();

  // todo: 同步通道
  let (tx, rx) = mpsc::sync_channel(0);

  let handle = thread::spawn(move || {
    println!("发送之前");
    tx.send(1).unwrap();
    println!("发送之后");
  });

  println!("睡眠之前");
  thread::sleep(Duration::from_secs(3));
  println!("睡眠之后");

  println!("receive {}", rx.recv().unwrap());
  handle.join().unwrap();
   */

  // todo: 同步缓存通道
  let (tx, rx) = mpsc::sync_channel(1);

  let handle = thread::spawn(move || {
    println!("首次发送之前");
    tx.send(1).unwrap();
    println!("首次发送之后");
    tx.send(2).unwrap();
    println!("再次发送之后");
    println!("主线程没有recv 这个2");
  });

  println!("睡眠之前");
  thread::sleep(Duration::from_secs(3));
  println!("睡眠之后");

  println!("receive {}", rx.recv().unwrap());
  handle.join().unwrap();
}


