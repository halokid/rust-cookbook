extern crate consul_rs;

use consul_rs::Client;

#[test]
fn kv() {
  let c = Client::new("10.87.134.91", 32350);

  let ok = c.kv_put("test-key", "test_value").unwrap();
  assert_eq!(ok, true);

  let pairs = c.kv_get("test-key").unwrap();
  let pair = &pairs[0];
  let v = pair.get_value().unwrap();
  assert_eq!(b"test_value"[..].to_vec(), v);

//  let ok = c.kv_delete("test-key").unwrap();
//  assert_eq!(ok, true);

  let ok = c.kv_put("pomid/rust-test", "rust-test").unwrap();
  assert_eq!(ok, true);
  let ok = c.kv_put("pomid/rust-test/tcp@8.8.8.8:9999", "typ=rust").unwrap();
  assert_eq!(ok, true);
}

