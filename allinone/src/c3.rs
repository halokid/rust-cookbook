use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::{rc, thread};
use std::sync::mpsc::channel;

/// error[E0382]: use of moved value: `counter`
//   --> examples/c3.rs:9:32
//    |
// 5  |   let counter = Mutex::new(0);
//    |       ------- move occurs because `counter` has type `Mutex<i32>`, which does not implement the `Copy` trait
// ...
// 9  |     let handle = thread::spawn(move || {
//    |                                ^^^^^^^ value moved into closure here, in previous iteration of loop
// 10 |       let mut num = counter.lock().unwrap();
//    |                     ------- use occurs due to use in closure
//
// For more information about this error, try `rustc --explain E0382`.
// error: could not compile `NeonRabbitGW` due to previous error
///
/*
fn error1() {
  let counter = Mutex::new(0);
  let mut handles = vec![];

  for _ in 0..10 {
    let handle = thread::spawn(move || {
      let mut num = counter.lock().unwrap();

      *num += 1;
    });

    handles.push(handle);
  }

  for handle in handles {
    handle.join().unwrap();
  }

  println!("Result -->>> {}", *counter.lock().unwrap());
}
 */

fn c1() {
  // let counter = Rc::new(Mutex::new(0));
  let counter = Arc::new(Mutex::new(0));
  let mut handles = vec![];

  for _ in 0..10 {
    // let counter = Rc::clone(&counter);
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
      let mut num = counter.lock().unwrap();

      *num += 1;
      // TODO: the principle of `mutex`, when the mutex lock, will lock always until the
      // TODO: locked vaiable leaving Scope, like this finished once for loop
      // TODO: so the `mutex` is not good for the concurrent program
    });

    handles.push(handle);
  }

  for handle in handles {
    handle.join().unwrap();
  }

  println!("Final counter:  -->>> {}", *counter.lock().unwrap());
}

const N: usize = 10;

fn c2() {
  let data = Arc::new(Mutex::new(0));
  let (tx, rx) = channel();

  for i in 0..20 {
    println!("i -->>> {}", i);
    let (data, tx) = (data.clone(), tx.clone());
    thread::spawn(move || {
      let mut num = data.lock().unwrap();
      *num += 1;

      if *num == N {
        // tx.send(()).unwrap();
        tx.send(*num).unwrap();
      }

      // TODO: the lock is unlocked here when `data` goes out of scope
    });
  }

  let x = rx.recv().unwrap();
  println!("x -->>> {:?}", x);
  println!("data -->>> {:?}, {:?}, {:?}", data, data.lock(), data.try_lock());

  // TODO: will block, because lock repeat, so thread will dead lock
  // println!("data -->>> {:?}, {:?}, {:?}", data, data.lock(), data.lock());
}

fn main() {
  c2()
}








