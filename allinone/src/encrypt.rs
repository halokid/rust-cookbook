
extern crate rustc_serialize;
use rustc_serialize::hex::{FromHex, ToHex};
use self::rustc_serialize::base64::{ToBase64, FromBase64};

extern crate base64;
use std::u8;
use self::base64::{encode};

pub fn hex_to_base64(hex: String) -> String {

  // Make vector of bytes from octets
  let mut bytes = Vec::new();
  for i in 0..(hex.len()/2) {
    let res = u8::from_str_radix(&hex[2*i .. 2*i+2], 16);
    match res {
      Ok(v) => bytes.push(v),
      Err(e) => println!("Problem with hex: {}", e),
    };
  };

  encode(&bytes) // now convert from Vec<u8> to b64-encoded String
}

pub fn comm1() {
  // 字符串转为[]byte
  // let s = "foo";
  let s = r#"{"userId":"xxxxx@xxxxx.com","class":"00111111"}"#;
  let sb = s.as_bytes();
  println!("sb ------ {:?}", sb);

  // []byte 转化为 16进制
  let foo_hex = s.as_bytes().to_hex();
  println!("foo_hex(16进制) ---------- {}", foo_hex);

  // 16进制转化为 []byte
  let foo_bs = foo_hex.from_hex().unwrap();
  println!("foo_bs ---------- {:?}", foo_bs);

  //  16进制用base64 encode
  let res = foo_hex.from_hex().unwrap().to_base64(rustc_serialize::base64::STANDARD);
  println!("res 1 ----- {}", res);

  let res = foo_hex.from_hex().unwrap().as_slice().to_base64(rustc_serialize::base64::STANDARD);
  println!("res 2 ----- {}", res);

  let res = hex_to_base64(foo_hex);
  println!("res 3 ----- {}", res);

  let res1 = res.from_base64();
  let res1 = base64::decode(res).unwrap();      // base64 decode
  println!("res 4 ----- {}", res);
}



