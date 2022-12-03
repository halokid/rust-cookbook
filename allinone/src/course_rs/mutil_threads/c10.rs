use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

enum Fruit {
  Apple(u8),
  Orange(String)
}

pub fn comm() {
  let (tx, rx): (Sender<Fruit>, Receiver<Fruit>) = mpsc::channel();

  tx.send(Fruit::Orange("sweet".to_string())).unwrap();
  tx.send(Fruit::Apple(2)).unwrap();

  for _ in 0..2 {
    match rx.recv().unwrap() {
      Fruit::Apple(count) => {
        println!("received {} apple", count);
      }

      Fruit::Orange(flavor) => {
        println!("received {} orages", flavor);
      }
    }
  }
}