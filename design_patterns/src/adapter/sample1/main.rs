use crate::sample1::adaptee::SpecificTarget;
use crate::sample1::adapter::TargetAdapter;
use crate::sample1::target::{OrdinaryTarget, Target};

fn call(target: impl Target) {
  println!("{}", target.request());
}

fn main() {
  let target = OrdinaryTarget;

  print!("A compatible target can be directly called: ");
  call(target);

  let adaptee = SpecificTarget;
  println!(
    "Adaptee is incompatible with client: {}", adaptee.specific_request()
  );

  let adapter = TargetAdapter::new(adaptee);
  println!("But with adapter client can call its method: ");
  call(adapter);
}