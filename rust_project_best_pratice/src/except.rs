/*
程序编译异常情况汇总
*/
use serde_json::{Value, Result};
use serde_json::json;

/*
pub fn ex1() -> String {
  // 异常范例: annot use the `?` operator in a function that returns `std::string::String`
  let json = r#"
  {
    "name": "halokid",
  }"#;

  let v: Value = serde_json::from_str(json)?;

  println!("name = {}", v["name"]);
  return v["name"].to_string();

//  Ok(())
}
*/

pub fn ex1_fix(change_str: &mut String) -> Result<()> {
  let json = r#"
  {
    "name": "halokid",
    "age": 18
  }"#;

  let v: Value = serde_json::from_str(json)?;

  println!("name = {}", v["name"]);
  let name = json!(v["name"]);
  let rsp = "hello, ".to_string() + name.as_str().unwrap();
  println!("rsp: {}", rsp);
  change_str.push_str(&rsp.to_string());

  Ok(())
}