
/*
异步通道channel
 */
use std::sync::mpsc;
use std::thread;

// thread count
const THREAD_COUNT: i32 = 2;

pub fn comm() {
  // create a channel
  let (tx, rx): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = mpsc::channel();

  // create thread for send message
  for id in 0..THREAD_COUNT {
    // notice the Sender can be clone, then support multi Sender process
    let thread_tx = tx.clone();
    thread::spawn(move || {
      // send a message, here is number id
      thread_tx.send(id + 1).unwrap();
      println!("send {}", id + 1);
    });
  }

  thread::sleep_ms(2000);
  println!("wake up neo...");
  // receive sub thread message in parent thread and print the message
  for _ in 0..THREAD_COUNT {
    println!("receive: {}", rx.recv().unwrap());
  }
}



