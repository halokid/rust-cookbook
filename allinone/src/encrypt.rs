
extern crate rustc_serialize;
use rustc_serialize::hex::{FromHex, ToHex};
use self::rustc_serialize::base64::ToBase64;

pub fn comm1() {
  // 字符串转为[]byte
  // let s = "foo";
  let s = r#"{"userId":"xxxxx@xxxxx.com","class":"00111111"}"#;
  let sb = s.as_bytes();
  println!("sb ------ {:?}", sb);

  // []byte 转化为 16进制
  let foo_hex = s.as_bytes().to_hex();
  println!("foo_hex(16进制) ---------- {}", foo_hex);

  let foo_bs = foo_hex.from_hex().unwrap();
  println!("foo_bs ---------- {:?}", foo_bs);


  // let res = foo_hex.from_hex().unwrap().to_base64(rustc_serialize::base64::STANDARD);
  let res = foo_hex.from_hex().unwrap().as_slice().to_base64(rustc_serialize::base64::STANDARD);
  println!("res ----- {}", res);
}