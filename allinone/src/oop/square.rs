use crate::oop::comm::HasArea;

pub struct Square {
  pub(crate) x: f64,
  pub(crate) y: f64,
  pub(crate) side: f64,
}

impl HasArea for Square {
  fn area(&self) -> f64 {
    self.side * self.side
  }
}