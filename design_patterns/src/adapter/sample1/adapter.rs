use crate::sample1::adaptee::SpecificTarget;
use crate::sample1::target::Target;

/// TODO: converts adaptee's specific interface to a compatible `Target` output
pub struct TargetAdapter {
  adaptee: SpecificTarget,
}

impl TargetAdapter {
  pub fn new(adaptee: SpecificTarget) -> Self {
    Self {
      adaptee,
    }
  }
}

impl Target for TargetAdapter {
  fn request(&self) -> String {
    // here's the `adaptation` of a specific output to a compatible output
    self.adaptee.specific_request().chars().rev().collect()
  }
}


