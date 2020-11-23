use lazy_static::*;
use std::collections::HashMap;

// pub const CONSUL_ADDR: String = String::from("10.87.134.91");
// pub const CONSUL_PORT: u16 = 32350;

lazy_static! {

    #[derive(Debug)]
    pub static ref CFG: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("aa", "foo");
        m
    };
}

#[test]
fn test_cfg1() {
  println!("{:?}", CFG);

  // for (k, v) in &*CFG {
  //   println!("{}: {}", k, v);
  // }

  for (k, v) in &*CFG {
    println!("{}: {}", k, v);
  }
}






