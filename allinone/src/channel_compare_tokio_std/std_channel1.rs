
use std::sync::mpsc;
use std::thread;

pub fn comm() {
  let (tx, rx): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();

  // thread spawn send
  thread::spawn(move || {
    tx.send("hello".to_string()).unwrap();
  });

  // recv from spawn
  let s = rx.recv().unwrap();
  println!("1. recv from thred spawn --- {}", s);

  // loop
  let (tx, rx): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = mpsc::channel();
  for i in 0..9 {
    let tx_cl = tx.clone();
    thread::spawn(move || {
      tx_cl.send(i).unwrap()
    });
  }
  for _ in 0..9 {
    let sx = rx.recv().unwrap();
    println!("2. loop recv from thred spawn --- {}", sx);
  }
}


