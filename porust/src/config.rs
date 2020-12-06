use lazy_static::*;
use std::collections::HashMap;

// pub const CONSUL_ADDR: String = String::from("10.87.134.91");
// pub const CONSUL_PORT: u16 = 32350;

lazy_static! {

    #[derive(Debug)]
    pub static ref CFG: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        let env = "dev";
        m.insert("env", env);
        if env == "test" {
          println!("------ 测试环境 ------");
          m.insert("consul_addr", "10.87.134.91");
          m.insert("consul_port", "32350");
        } else if env == "prd" {
          println!("------ 生产环境 ------");
          m.insert("consul_addr", "10.87.134.91");
          m.insert("consul_port", "32350");
        } else if env == "dev" {
          println!("------ 开发环境 ------");
          m.insert("consul_addr", "127.0.0.1");
          m.insert("consul_port", "8500");
        }

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






