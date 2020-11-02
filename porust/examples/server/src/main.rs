extern crate porust;

fn main() {
  println!("start run porust server...");
//  porust::test();

  let addr = String::from("127.0.0.1:18080");
  porust::server::run(&addr);
//  porust::server::run(addr);
}
