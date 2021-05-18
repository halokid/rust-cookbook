extern crate base64;

use std::str;
use base64::{encode, decode};

pub fn comm() {
  // let hello = b"hello rustaceans";
  let hello = r#"{"userId":"xxxxx@xxxxx.com","class":"00111111"}"#;
  let encoded = encode(hello);
  let decoded = decode(&encoded).unwrap();

  println!("hello -------- {}", hello);
  println!("encode -------- {}", encoded);
  println!("decode -------- {:?}", str::from_utf8(&decoded).unwrap());

  // println!("origin: {}", str::from_utf8(hello).unwrap());
  // println!("origin: {}", str::from_8(hello).unwrap());
  // println!("base64 encoded: {}", encoded);
  // println!("back to origin: {}", str::from_utf8(&decoded).unwrap());
//
}

