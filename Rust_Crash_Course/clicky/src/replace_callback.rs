use std::thread::{ sleep, spawn };
use std::time::Duration;
use std::sync::{ Arc, Mutex };

pub fn comm() {
  let mut msg = String::from("Fearless");
  for _ in 1..11 {
    msg.push('!');
    println!("{}", msg);
    sleep(Duration::new(1, 0));
  }
}

pub fn comm2() {
  let mut msg = String::from("Fearless");
  for _ in 1..11 {
    let mut inner = || {
      msg.push('!') ;
      println!("{}", msg);
    };

    inner();
    sleep(Duration::new(1, 0));
  }
}

pub fn comm3() {
  let msg = Arc::new(Mutex::new(String::from("Fearless")));
  for _ in 1..11 {
    let mut msg = msg.clone();
    let mut inner = move || {
      let msg = msg.lock();
      // msg.push('!');
      // println!("{}", msg);
    };
    spawn(inner);
    sleep(Duration::new(1, 0));
  }
}





