use core::fmt;
use std::fmt::Formatter;
use std::ops::Add;

struct Meters(u32);

impl fmt::Display for Meters {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "distination distance from you {} meters", self.0)
  }
}

impl Add for Meters {
  type Output = Self;

  fn add(self, other: Meters) -> Self {
    Self(self.0 + other.0)
  }
}

pub fn comm() {
  let d = calculate_distance(Meters(10), Meters(20));
  println!("{}", d);

  // ------------------------------------------------
  type Metersx = u32;
  let x: u32 = 5;
  let y: Metersx = 5;
  println!("x + y = {}", x + y);
}

fn calculate_distance(d1: Meters, d2: Meters) -> Meters {
  d1 + d2
}


