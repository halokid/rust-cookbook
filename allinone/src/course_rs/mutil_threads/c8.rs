use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// todo: 使用 for 进行循环接收
pub fn comm() {
  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
    let vals = vec![
      String::from("hi"),
      String::from("from"),
      String::from("the"),
      String::from("thread"),
    ];

    for val in vals {
      tx.send(val).unwrap();
      thread::sleep(Duration::from_secs(1));
    }
  });

  for received in rx {
    println!("Got: {}", received);
  }
}

