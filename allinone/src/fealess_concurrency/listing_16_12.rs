use std::sync::Mutex;

pub fn comm() {
  let m = Mutex::new(5);

  {
    let mut num = m.lock().unwrap();
    *num = 6
  }

  println!("m = {:?}", m);
}