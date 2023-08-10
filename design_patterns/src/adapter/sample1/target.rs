
/// TODO: Target is the same interface of every adaptee, it usually is behavior
/// TODO: like below `request` is behavior
pub trait Target {
  fn request(&self) -> String;
}

pub struct OrdinaryTarget;

impl Target for OrdinaryTarget {
  fn request(&self) -> String {
    "Ordinary request.".into()
  }
}

