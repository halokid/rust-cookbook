
extern crate easy_hasher;

use easy_hasher::easy_hasher::sha256;

#[test]
fn test_sha512() {
  let s = "hello".to_string();
  let hash = sha256(&s);
  let s_hash = hash.to_hex_string();
  println!("s_hash -- {}", s_hash);
}