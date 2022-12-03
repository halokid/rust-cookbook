use std::thread;
use std::time::Duration;

pub fn comm() {
  // create thread A
  let new_thread = thread::spawn(move || {
    // create thread B again
    thread::spawn(move || {
      loop {
        println!("I am a new thread.");
      }
    });
    println!("-->>> Thread A done.");
  });

  // wait new create threads done.
  new_thread.join().unwrap();
  println!("Child thread is finished!");

  // sleep for a while, check child thread is running or not?
  thread::sleep(Duration::from_millis(1));
}