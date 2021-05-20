/*
extern crate crypto;
extern crate rand;
extern crate rustc_serialize;

use crypto::aes::{self, KeySize};
use crypto::digest::Digest;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::Sha256;
use crypto::symmetriccipher::SynchronousStreamCipher;

use rustc_serialize::base64::{STANDARD, ToBase64};
use rustc_serialize::hex::ToHex;

use std::iter::repeat;
use rand::{OsRng, Rng};

pub fn comm() {
  let mut gen = OsRng::new().expect("Failed to get OS random generator");
  let mut hmac_key: Vec<u8> = repeat(0u8).take(32).collect();
  gen.fill_bytes(&mut hmac_key);
  let message = "Ceterum censeo Carthaginem esse delendam";
  println!("Message: {}", message);
  println!("HMAC key: {}", hmac_key.to_base64(STANDARD));
  let mut hmac = Hmac::new(Sha256::new(), &hmac_key);
  hmac.input(message.as_bytes());
  println!("HMAC digest: {}", hmac.result().code().to_hex());

  let enc = hmac.result().code().to_hex();
  println!("enc --------------- {}", enc);


  hmac.input(enc.as_bytes());
  let xx = hmac.result().code().to_hex();
  println!("{:?}", xx);
}

 */




