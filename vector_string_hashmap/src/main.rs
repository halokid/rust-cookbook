use std::collections::HashMap;

fn main() {
//  let mut v: Vec<i32> = Vec::new();
//  let mut v = Vec::new();

//  v.push(5);
//  v.push(6);
//  v.push(7);
//  v.push(8);

  /**
  let v = vec![1, 2, 3, 4, 5];
  let third: &i32 = &v[2];
  println!("The third element is {}", third);

  match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element"),
  }
  */

  let mut v = vec![1, 2, 3, 4, 5];
//  let first = &v[0];
  v.push(6);      // todo: push了之后， 因为超过了原来的空间， 所以第一个元素的内存位置可能会更换， 所以再引用first的时候， 可能会造成错误
//  println!("The first element is {}", first);

  println!("The first element is {}", &v[0]);
  println!("The first element is {}", v[0]);
  println!("The old last element is {}", v[4]);
  println!("The new last element is {}", v[5]);

//  for i in v {
//    println!("{}", i);
//  }

  for i in &v {
    println!("{}", i);
  }

//  let hello = String::from("السلام عليكم");
//  let hello = String::from("Dobrý den");
//  let hello = String::from("Hello");
//  let hello = String::from("שָׁלוֹם");
//  let hello = String::from("नमस्ते");
//  let hello = String::from("こんにちは");
//  let hello = String::from("안녕하세요");
//  let hello = String::from(":q
// 你好");
//  let hello = String::from("Olá");
//  let hello = String::from("Здравствуйте");
//  let hello = String::from("Hola");

  let field_name = String::from("Favorite color");
  let field_value = String::from("Blue");

  let mut map = HashMap::new();
  map.insert(field_name, field_value);

//  println!("field_name is {}", field_name);
  for (key, value) in &map {
    println!("{}: {}", key, value);
  }

//  map.insert(field_name, "Red".to_string());
  map.insert(String::from("Favorite color"), "Red".to_string());

  println!("{:?}", map);
}





