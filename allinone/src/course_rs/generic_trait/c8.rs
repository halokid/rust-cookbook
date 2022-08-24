use std::fmt;
use std::fmt::{Display, Formatter};

// todo: feature constraint, if you want implement some trait, you must impl
// todo: the constriant trait of it
trait OutlinkPrint: Display {
  fn outline_print(&self) {
    let output = self.to_string();
    let len = output.len();
    println!("{}", "*".repeat(len + 4));
    println!("*{}*", " ".repeat(len + 2));
    println!("* {} *", output);
    println!("*{}*", " ".repeat(len + 2));
    println!("{}", "*".repeat(len + 4));
  }
}

struct Point {
  x: i32,
  y: i32,
}

// todo: must impl the `Display` trait if want to impl `OutlinkPrint`
impl Display for Point {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}

impl OutlinkPrint for Point {}

// ---------------------------------------------------------
// todo: impl extanal trait use `newtype`
// todo: example, if we want to change the the default `Display` impl trait of `Vec`
// todo: we can define a `newtype` name Wrapper
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "[{}]", self.0.join(", "))
  }
}

pub fn comm() {
  let w = Wrapper(vec![String::from("hello"), String::from("world")]);
  println!("w = {}", w);
}



