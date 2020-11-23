/*
利用 lazy_static 的方式获取配置, 只在读取的时候初始化
 */

use lazy_static::*;
use std::collections::HashMap;

lazy_static! {
    pub static ref HASHMAP: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
        m
    };
}


pub fn comm() {
  println!("The entry for `0` is \"{}\".", HASHMAP.get(&0).unwrap());
  println!("The entry for `1` is \"{}\".", HASHMAP.get(&1).unwrap());
}