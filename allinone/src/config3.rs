/*
lazy_static 的方式来定义， 可以用 enum 来定义数据的不同类型
 */

use std::fmt;
use lazy_static::*;
use std::collections::HashMap;
use std::borrow::Borrow;

#[derive(Debug)]
enum Value {
  Str(&'static str),
  Int(i32),
}

/*
impl fmt::Display for Value {
  // This trait requires `fmt` with this exact signature.
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    // Write strictly the first element into the supplied output
    // stream: `f`. Returns `fmt::Result` which indicates whether the
    // operation succeeded or failed. Note that `write!` uses syntax which
    // is very similar to `println!`.
    write!(f, "{:?}", self)
  }
}
 */

lazy_static! {
    static ref HASHMAP: HashMap<u32, Value> = {
        let mut m = HashMap::new();
        m.insert(0, Value::Str("foo"));
        m.insert(1, Value::Str("bar"));
        m.insert(2, Value::Str("baz"));
        m.insert(3, Value::Int(99));
        m
    };
    static ref COUNT: usize = HASHMAP.len();
    // static ref NUMBER: u32 = times_two(21);
}


pub fn comm() {
  // println!("config3 \"{}\".", HASHMAP.get(&0).unwrap());
  let val = HASHMAP.get(&0).unwrap();
  match val {
    Value::Str(c) => {
      println!("key 0: {}", c);
    }
    _ => {}
  }

  let val = HASHMAP.get(&3).unwrap();
  match val {
    Value::Int(c) => {
      println!("key 3: {}", c);
    }
    _ => {}
  }

  // println!("val: {}", val_str::Str);
  // println!("config3 \"{}\".", HASHMAP.get(&0).unwrap());
}