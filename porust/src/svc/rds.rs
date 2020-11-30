/*
rds service
 */

extern crate redis;
use redis::Commands;
use self::redis::{RedisResult, RedisError};

fn read_data(reqdata_data: &serde_json::Value, handled_rsp: &mut String) -> serde_json::Result<()> {
  // {"call": "read_data", "data": {"key": "halokid"}}
  println!("redis read_data handle: {}", reqdata_data);
  let key = serde_json::json!(reqdata_data["key"]).as_str().unwrap();
  // get

  Ok(())
}

// fn get() {
  /*
  如果是以 问好结尾的话:
  let client = redis::Client::open("redis://172.20.71.25")?;
  则直接影响了函数的返回类型
   */
// }

#[test]
fn test_read_data() {
  // let data = get();
  // println!("data: {}", data);

  let client = redis::Client::open("redis://:mypassword@127.0.0.1:6379/0", ).unwrap();
  let mut con = client.get_connection().unwrap();

  // get key
  let val: String = con.get("halokid-xxx").unwrap();    // todo: 以 unwrap 结尾的封装， 可以返回正确取值的任何类型， 值是什么类型就返回什么类型
  println!("get val: {}", val);

  // search key
  let val: Vec<String> = redis::cmd("KEYS").arg("*").query(&mut con).unwrap();
  println!("val {}", val[0]);

  let mut i: i32 = 0;
  con.scan_match("*").unwrap().for_each(|s: String| {
    println!("i: {}, s: {}", i, s);
    i += 1;
  });

  let iter = con.scan_match("*").unwrap();
  // println!("iter {}", iter);

  // todo: 匹配迭代器的元素类型
  for x in iter {
    let x: String = x;
    println!("x {}", x);
  }

  let iter = con.scan_match("*").unwrap();
  for y in iter {
    let y: usize = y;
    println!("y {}", y);
  }

}



