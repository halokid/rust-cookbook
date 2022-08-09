use std::fmt::{Debug, Display};

fn report<T: Debug + Display>(item: T) {
  println!("{:?}", item);
  println!("{}", item);
}

pub fn comm() {
  report("hello");
}