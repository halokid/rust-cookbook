use std::net::IpAddr;

pub fn comm() {
  let v = vec![1, 2, 3];

  // v[99];

  // -----------------------------------------------
  // let host: IpAddr = "127.0.0.1x".parse().unwrap();
  let host: IpAddr = "127.0.0.1".parse().unwrap();
  println!("host -->>> {}", host);
}


