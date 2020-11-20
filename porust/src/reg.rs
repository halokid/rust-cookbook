use consul_rs::Client;

// pub mod config;
#[path = "config.rs"] mod config;

// 服务注册
pub fn regiser(svc_addr: &String) -> bool {
  let svc = String::from("porust");
  let key = format!("pomid/{}/tmp@{}", svc, svc_addr);
  let val = String::from("typ=rust");

  let c = Client::new(config::CONSUL_ADDR, config::CONSUL_PORT);
  let ok = c.kv_put(key, val).unwrap();
  ok
}



