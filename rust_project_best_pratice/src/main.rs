pub mod server;
pub mod serialize;
pub mod except;

fn main() {
  println!("Hello, world!");

//  serialize::run_serial();

//  except::ex1();
  let mut change_str = String::new();
  except::ex1_fix(&mut change_str);
  println!("xx: {}", change_str);
}
