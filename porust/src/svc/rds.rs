/*
rds service
 */

extern crate redis;
use redis::Commands;

fn read_data(reqdata_data: &serde_json::Value, handled_rsp: &mut String) -> serde_json::Result<()> {
  // {"call": "read_data", "data": {"key": "halokid"}}
  println!("redis read_data handle: {}", reqdata_data);
  let key = serde_json::json!(reqdata_data["key"]).as_str().unwrap();
  // get

  Ok(())
}

fn get() -> String {
  /*
  如果是以 问好结尾的话:
  let client = redis::Client::open("redis://172.20.71.25")?;
  则直接影响了函数的返回类型
   */
  let client = redis::Client::open("redis://172.20.71.25:6379/", ).unwrap();
  let mut con = client.get_connection().unwrap();
  let val = con.get("halokid-*").unwrap();    // todo: 以 unwrap 结尾的封装， 可以返回正确取值的任何类型， 值是什么类型就返回什么类型
  return val;
}

#[test]
fn test_read_data() {
  let data = get();
  println!("data: {}", data);
}


