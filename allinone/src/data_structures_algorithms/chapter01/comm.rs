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

}








