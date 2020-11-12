/*
程序编译异常情况汇总
*/
use serde_json::{Value, Result};

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

pub fn ex1_fix(change_str: &String) -> Result<()> {
  let json = r#"
  {
    "name": "halokid",
  }"#;

  let v: Value = serde_json::from_str(json)?;

  println!("name = {}", v["name"]);

  Ok(())
}