
trait Area {
  fn area(&self) -> f64;
}

#[derive(Debug)]
struct Circle {
  radius:   f64,
}

#[derive(Debug)]
struct Rectangle {
  width:  f64,
  height: f64,
}

impl Area for Circle {
  fn area(&self) -> f64 {
    use std::f64::consts::PI;
    PI * self.radius.powf(2.0)
  }
}

impl Area for Rectangle {
  fn area(&self) -> f64 {
    self.width * self.height
  }
}

pub fn comm() {
  let circle = Circle{ radius: 5.0 };
  let rectangle = Rectangle {
    width:    10.0,
    height:   20.0,
  };

  println!("circle {:?}", circle);
  println!("rectangle {:?}", rectangle);

}

// ------------------------------------------------------
#[derive(Debug, PartialEq)]
struct Point {
  x:  i32,
  y:  i32,
}

use std::fmt;

// todo: 自己实现 Point 的 fmt::Display 特性
impl fmt::Display for Point {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}, {}", self.x, self.y)
  }
}

pub fn comm2() {
  let p1 = Point{ x: 1, y: 2 };
  let p2 = Point{ x: 4, y: -3 };

  if p1 == p2 {
    println!("equal");
  } else {
    println!("not equal");
  }

  // todo: {}的输出是 Display 特性
  println!("{}", p1); // can't print using the '{}' format specifier!

  // todo: {:?}的输出是Debug特性， 不是Display特性
  println!("{:?}", p1); //  can't print using the '{:?}' format specifier!
}















