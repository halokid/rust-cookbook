
struct Circle {
  x: f64,
  y: f64,
  radius: f64,
}

impl Circle {
  fn area(&self) -> f64 {
    std::f64::consts::PI * (self.radius * self.radius)
  }
}

pub fn comm() {
  let c = Circle{
    x: 0.0,
    y: 0.0,
    radius: 2.0
  };
  println!("Circle area: {}", c.area());
}


