use crate::oop::comm::HasArea;

pub struct Circle {
  pub(crate) x: f64,
  pub(crate) y: f64,
  pub(crate) radius: f64,
}

impl HasArea for Circle {
  fn area(&self) -> f64 {
    std::f64::consts::PI * (self.radius * self.radius)
  }
}

