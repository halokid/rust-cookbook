use serde_json::{Result, Value};

pub fn run_serial() -> Result<()> {
  println!("call run_serial...");
  let json = r#"
  {
    "name": "halokid",
    "age": 18
  }"#;

  let v: Value = serde_json::from_str(json)?;

  println!("name = {}", v["name"]);
  println!("age = {}", v["age"]);
//  println!("blog = {}", v["blog"]);

  Ok(())
}


