
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
