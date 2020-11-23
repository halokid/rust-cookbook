#[path = "config.rs"] pub mod config;
#[path = "config2.rs"] pub mod config2;

pub fn comm() {
  println!("read config for `0` is \"{}\".", config::HASHMAP.get(&0).unwrap());
  println!("read config for `1` is \"{}\".", config::HASHMAP.get(&1).unwrap());

  unsafe {
    println!("confin2 attr: {}", config2::NUM);
  }

  // 初始化配置
  // config2::init();
  // unsafe {
  //   println!("CFG.name {}", config2::CFG.name);
  //   println!("CFG.age {}", config2::CFG.age);
  // }
}