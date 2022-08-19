use std::fmt::{Display, Formatter};
use std::ops::Add;

// derive Debug trait for Point struct, use in format output
#[derive(Debug)]
struct Point<T: Add<T, Output = T>> {
  x:  T,
  y:  T,
}

impl<T: Add<T, Output = T>> Add for Point<T> {
  type Output = Point<T>;

  fn add(self, p: Point<T>) -> Point<T> {
    Point {
      x: self.x + p.x,
      y: self.y + p.y,
    }
  }
}

fn add<T: Add<T, Output = T>>(a: T, b: T) -> T {
  a + b
}

// -----------------------------------------------
#[derive(Debug, PartialEq)]
enum FileState {
  Open,
  Closed,
}

#[derive(Debug)]
struct File {
  name:     String,
  data:     Vec<u8>,
  state:    FileState,
}

impl Display for FileState {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match *self {
      FileState::Open => write!(f, "--- OPEN"),
      FileState::Closed => write!(f, "--- CLOSED"),
    }
  }
}

impl Display for File {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "-->>> <{} ({})>", self.name, self.state)
  }
}

impl File {
  fn new(name: &str) -> File {
    File {
      name: name.to_string(),
      data: Vec::new(),
      state: FileState::Closed,
    } 
  } 
}

pub fn comm() {
  let p1 = Point {
    x: 1.1f32,
    y: 1.1f32,
  };
  let p2 = Point {
    x: 2.1f32,
    y: 2.1f32,
  };
  println!("{:?}", add(p1, p2));

  // -----------------------------------------
  let f6 = File::new("f6.txt");
  println!("{:?}", f6);
  println!("{}", f6);
}






