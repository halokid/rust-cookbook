extern crate porust;

fn main() {
//  porust::test();
  let addr = "127.0.0.1:18080";
  println!("start run porust server: {}", addr);
  let addr = String::from("127.0.0.1:18080");
  porust::server::run(&addr);
//  porust::server::run(addr);
}
