
use std::env;

pub fn comm() {
  let args: Vec<_> = env::args().collect();
  println!("{:?}", args);
}

