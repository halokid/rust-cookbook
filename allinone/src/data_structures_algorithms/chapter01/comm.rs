use std::thread;
use std::sync::{Mutex, Arc};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use serde::private::de::IdentifierDeserializer;

use crate::data_structures_algorithms::chapter01::door;

fn shared_state() {
  // todo: the origin varible share in different threads
  let v = Arc::new(Mutex::new(vec![]));
  let handles = (0..10).map(|i| {

    // todo: the middleware varible(point reference) as bridge between other
    // todo: threads and main thread
    let numbers = Arc::clone(&v);

    thread::spawn(move || {
      let mut vector = numbers.lock().unwrap();
      (*vector).push(i);
    })
  });

  for hanle in handles {
    hanle.join().unwrap();
  }
  println!("v -->>> {:?}", *v.lock().unwrap());
}

fn threading() {
  let x = 10;
  let handle = thread::spawn(move || {
    println!("get number from a thread, number is -->>> {}, thredID -->>> {:?}",
      x, thread::current().id());
  });
  handle.join().unwrap();
}

fn channels() {
  const N: i32 = 10;
  let (tx, rx): (Sender<i32>, Receiver<i32>) = channel();

  let handles = (0..N).map( |i| {
    // todo: iterator map fn do
    let _tx = tx.clone();
    thread::spawn(move || {
      // dont use the result
      let _ = _tx.send(i).unwrap();
    })
  });

  // run all threads
  for h in handles {
    h.join().unwrap();
  }

  // receive N times
  let numbers: Vec<i32> = (0..N).map( |_| rx.recv().unwrap()).collect();

  println!("numbers -->>> {:?}", numbers);
}

#[derive(Debug, Clone)]
struct FileName {
  name: Rc<String>,
  ext:  Rc<String>,
}

fn ref_counter() {
  let name = Rc::new(String::from("main"));
  let ext = Rc::new(String::from("rs"));

  for _ in 0..3 {
    let f = FileName {
      name:  name.clone(),
      ext:   ext.clone()
    };
  }
}









