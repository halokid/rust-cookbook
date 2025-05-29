use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};
use std::{rc, thread};
use std::sync::mpsc::channel;
use std::time::Duration;

// reference:  https://www.bookstack.cn/read/RustPrimer/rcarc-mutex.md

///
/// error[E0382]: use of moved value: `counter`
//   --> examples/c4.rs:14:32
//    |
// 10 |   let counter = RwLock::new(9);
//    |       ------- move occurs because `counter` has type `RwLock<i32>`, which does not implement the `Copy` trait
// ...
// 14 |     let handle = thread::spawn(move || {
//    |                                ^^^^^^^ value moved into closure here, in previous iteration of loop
// 15 |       let r1 = counter.read().unwrap();
//    |                ------- use occurs due to use in closure
//
// For more information about this error, try `rustc --explain E0382`.
/*
fn c1() {
  // let counter = Arc::new(RwLock::new(9));
  let counter = RwLock::new(9);
  let mut handles = vec![];

  for _ in 0..10 {
    let handle = thread::spawn(move || {
      let r1 = counter.read().unwrap();
      println!("r1 -->>> {}", r1);
    });

    handles.push(handle);
  }

  for handle in handles {
    handle.join().unwrap();
  }
}
 */

fn c2(){
  let counter_ori = Arc::new(RwLock::new(0));
  // let counter = RwLock::new(9);
  let mut handles = vec![];

  // TODO: read 1
  for i in 0..10 {
    let counter = Arc::clone(&counter_ori);
    let handle = thread::spawn(move || {
      let r1 = counter.read().unwrap();

      // *r1 += 1;

      // if i == 5 {
      //   let mut k = counter.write().unwrap();
      //   *k += 3;
      //   println!("*k in c2 -->>> {}", *k);
      // }
      println!("r1 -->>> {}", r1);
    });

    handles.push(handle);
  }

  // TODO: write
  let counter2 = Arc::clone(&counter_ori);
  let handle2 = thread::spawn(move || {
    let mut k = counter2.write().unwrap();
    *k += 3;

    // thread::sleep(Duration::from_secs(5));
    println!("*k in c2 -->>> {}", *k);
  });
  handles.push(handle2);

  // TODO: read 2
  for i in 0..3 {
    let counter3 = Arc::clone(&counter_ori);
    let handle3 = thread::spawn(move || {
      // TODO: no block, will read in the last
      thread::sleep(Duration::from_secs(2));
      let r2 = counter3.read().unwrap();

      println!("r2 -->>> {}", r2);
    });

    handles.push(handle3);
  }

  // TODO: read and write 1
  let counter4 = Arc::clone(&counter_ori);
  let handle4 = thread::spawn(move || {
    {
      let r4 = counter4.read().unwrap();    // read
      println!("r4 -->>> {}", r4);
    }
    // println!("r4 -->>> {}", r4);

    {
      let mut j = counter4.write().unwrap();    // write
      *j += 3;
      println!("*j in c4 -->>> {}", *j);
    }
  });
  handles.push(handle4);

  // TODO: read and write 2
  let counter5 = Arc::clone(&counter_ori);
  let handle5 = thread::spawn(move || {
    let read_r5 = counter5.read().unwrap();
    println!("read_r5 -->>> {}", read_r5);
    drop(read_r5);  // TODO: if not `drop`, below `write` will always lock

    let mut write_r5 = counter5.write().unwrap();
    *write_r5 += 6;
  });
  handles.push(handle5);

  // ---------------------------------------------------------------
  for handle in handles {
    handle.join().unwrap();
  }
}

fn main() {
  let num = RwLock::new(5);

  // many reader locks can be help at once
  {
    let r1 = num.read().unwrap();
    let r2 = num.read().unwrap();
    println!("r1 -->>> {}, r2 -->>> {}", r1, r2);
  }   // read locks are dropped at this point

  // only one write lock may be held, however
  {
    let mut w = num.write().unwrap();
    *w += 1;
    println!("*w -->>> {}", *w);
  }   // write lock is dropped here

  // ------------------------------------------------------------
  // c1();
  c2();
}



