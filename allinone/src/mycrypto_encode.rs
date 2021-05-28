
use std::str;
use base64::{encode, decode};

// 同一个文件的全局变量， 共用变量
static INTERF1: &str =  "6d7kb3za39";
static INTERF2: &str =  "c560e7z988";

pub fn comm() {
  // /*
  let msg = r#"{"name":"xxxxxxxxhalokid", "class":"xxxxxxxxxxxxxxxxxx"}"#;
  // hex encode
  let msg_hex = hex::encode(msg);
  println!("msg_hex ---------------- {}", msg_hex);

  // reverse string
  let rever_msg_hex: String = msg_hex.chars().rev().collect();
  println!("rever_msg_hex ----------------- {}", rever_msg_hex);

  // add interference string
  let interf1 = "6d7kb3za39";
  let interf2 = "c560e7z988";
  let rever_msg_hex_im = rever_msg_hex + interf1 + interf2;
  println!("rever_msg_hex_im ---------------- {}", rever_msg_hex_im);

  // base64 encode
  let msg_im_by = rever_msg_hex_im.as_bytes();
  let msg_im_by_en = encode(msg_im_by);
  println!("msg_im_by_en -------------------- {}", msg_im_by_en);

  let msg_im_by_de = decode(&msg_im_by_en).unwrap();
  println!("msg_im_by_de ---------------- {}", str::from_utf8(&msg_im_by_de).unwrap());
   // */

  let s = r#"{"name":"xxxxxxxxhalokid", "class":"xxxxxxxxxxxxxxxxxx"}"#;
  println!("s ------------------ {}", s);

  let en = cyp_encode(s);
  println!("en ------------------ {}", en);

  let de = cyp_decode(en);
  println!("de -------------------- {}", de);
}

// cypto encode
fn cyp_encode(s: &str) -> String {
  // hex encode
  let msg_hex = hex::encode(s);

  // reverse string
  let rever_msg_hex: String = msg_hex.chars().rev().collect();
  // println!("rever_msg_hex ------------ {}", rever_msg_hex);

  // add interference string
  // let interf1 = "6d7kb3za39";
  // let interf2 = "c560e7z988";
  // let rever_msg_hex_im = rever_msg_hex + interf1 + interf2;
  let rever_msg_hex_im = rever_msg_hex + INTERF1 + INTERF2;

  // base64 encode
  let msg_im_by = rever_msg_hex_im.as_bytes();
  let msg_im_by_en = encode(msg_im_by);
  msg_im_by_en
}

// cypto decode
fn cyp_decode(s_en: String) -> String {
  // base64 decode
  let b64_de = decode(s_en).unwrap();

  // fix interfence
  let b64_de_s = String::from_utf8(b64_de).unwrap();
  let f1 = b64_de_s.replace(INTERF1, "");
  // println!("f1 ------------ {}", f1);
  let f2 = f1.replace(INTERF2, "");
  // println!("f2 ------------ {}", f2);

  // reverse string
  let rever_msg_hex_ori: String = f2.chars().rev().collect();
  // println!("rever_msg_hex_ori ------------ {}", rever_msg_hex_ori);

  let de = hex::decode(rever_msg_hex_ori).unwrap();
  String::from_utf8(de).unwrap()
}













