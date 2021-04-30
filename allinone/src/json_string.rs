
use serde_json;

pub fn comm() {
  let s1 = "halokid".to_string();
  println!("s1 --- {}", s1);

  let s2 = "halokid";
  println!("s2 --- {}", s2);

  // 长字符串
  let s3 = r#"{"name":"halokid"}"#;
  let js: serde_json::Value = serde_json::from_str(s3).unwrap();
  let name = serde_json::json!(js["name"]);
  println!("s3 name --- {}, {}", name, name.to_string());

  let name1 = name.as_str().unwrap();
  println!("name1 ------ {}", name1);

  // todo: 通过 serde_json 取得的json 有双引号的话， 就通过这种方式来去掉双引号
  let name3 = name1.to_string();
  println!("name3 ------ {}", name3);

  let name4 = name.as_str().unwrap().to_string();
  println!("name4 ------ {}", name4);

  let name2 = name.to_string();
  println!("name2 ------ {}", name2);
}