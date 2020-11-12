pub mod server;
pub mod serialize;
pub mod except;

fn main() {
  println!("Hello, world!");

//  serialize::run_serial();

//  except::ex1();
  let mut change_str;
  except::ex1_fix();
}
