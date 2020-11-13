//extern crate local_ip;
use local_ipaddress;

#[test]
fn get_ip() {
//  let ip = local_ip::get().unwrap();
//  println!("local ip: {}", ip.to_string());

  println!("{}", local_ipaddress::get().unwrap());
}
