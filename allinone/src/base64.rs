
extern crate base64;

use std::str;
use base64::{encode, decode};

pub fn comm() {
  // let hello = b"hello rustaceans";

  let s = "7b226e616d65223a22787878787878787868616c6f6b6964222c2022636c617373223a22787878787878787878787878787878787878227d".to_string();
  let hello = s.as_bytes();

  let encoded = encode(hello);
  let decoded = decode(&encoded).unwrap();

  println!("origin: {}", str::from_utf8(hello).unwrap());
  println!("base64 encoded: {}", encoded);
  println!("back to origin: {}", str::from_utf8(&decoded).unwrap());

}

