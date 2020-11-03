extern crate porust;

fn main() {
  println!("start run porust client...");

  let addr = String::from("http://127.0.0.1:18080");
  let req_data = r#"{"name": "halokid"}"#;
  let mut rsp = String::new();
  porust::client::invoke(addr,req_data.to_string(), &mut rsp);
  println!("rsp: {}", rsp);
}


