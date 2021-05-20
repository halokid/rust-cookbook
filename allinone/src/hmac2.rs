
extern crate hmac;
extern crate sha2;
extern crate base64;
extern crate hex;

use sha2::Sha256;
use hmac::{Hmac, Mac};
use rustc_serialize::hex::ToHex;
use rustc_serialize::base64::ToBase64;

pub fn comm() {
  type HmacSha256 = Hmac<Sha256>;

  let secret = "the shared secret key here";
  let message = "the message to hash here";

  let mut mac = HmacSha256::new_varkey(secret.as_bytes()).unwrap();

  mac.input(message.as_bytes());

  let hash_message = mac.result().code();

  // to lowercase hexits
  // hex::encode(&hash_message);
  let xx = hex::encode(&hash_message);
  println!("xx ------------ {}", xx);
  // let yy = hex::decode(xx).unwrap().to_hex();
  // println!("yy ------------ {:?}", yy);     // todo: 这样输出是会有双引号的 "4643978965ffcec6e6d73b36a39ae43ceb15f7ef8131b8307862ebc560e7f988"
  // println!("yy ------------ {}", yy);

  let zz = hex::decode("48656c6c6f20776f726c6421").unwrap();
  // let zz = hex::decode(xx.as_str()).unwrap();
  // let zz = hex::decode("4643978965ffcec6e6d73b36a39ae43ceb15f7ef8131b8307862ebc560e7f988").unwrap();
  println!("zz ---------- {:?}", zz);

  let tt = String::from_utf8(zz).unwrap();
  println!("tt -------- {}", tt);


  // let hh = hex::encode("Hello world!");
  let hh = hex::encode(r#"{"name":"xxxxxxxxhalokid", "class":"xxxxxxxxxxxxxxxxxx"}"#);
  println!("hh ----------- {}", hh);
  let kk = hex::decode(&hh).unwrap();
  println!("kk ------------ {:?}", kk);

  let gg = hex::decode(hh.as_str()).unwrap();
  println!("gg ------------- {:?}", gg);
  println!("gg string ------------- {}", String::from_utf8(gg).unwrap());


  // to base64
  base64::encode(&hash_message);

}




