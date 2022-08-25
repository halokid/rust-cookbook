



pub fn comm() {
  let mut v = vec![1, 2, 3, 4, 5];

  let third: &i32 = &v[2];
  println!("1 -- the third element is {}", third);

  match v.get(2) {
    Some(third) => println!("2 -- the third element is {}", third),
    None => println!("fuck the third!!! its none"),
  }

  // println!("collection cross the border {}", &v[100]);

  // ---------------------------------------------------
  // let first = &v[0];
  // v.push(6) ;
  // println!("the first element is {}", first);

  // ---------------------------------------------------
  for i in &v {
    println!("{}", i);
  }

  for i in &mut v {
    *i += 10;
  }
  println!("v -->>> {:?}", &v);

  // ---------------------------------------------------
  // todo: how to store different type element, solution one is use `enum`
  let v = vec![
    IpAddr::V4("127.0.0.1".to_string()),
    IpAddr::V6("::1".to_string()),
  ];

  for ip in v {
    show_addr(ip);
  }

  // --------------------------------------------------
  // todo: how to store different type element, solution two is use `characteristic trait`
  let v: Vec<Box<dyn IpAddrx>> = vec![
    Box::new(V4x("127.0.0.1".to_string())),
    Box::new(V6x("::1".to_string())),
  ];

  for ip in v {
    ip.display();
  }
}

#[derive(Debug)]
enum IpAddr {
  V4(String),
  V6(String)
}

fn show_addr(ip: IpAddr) {
  println!("ip -->>> {:?}", ip);
}

// ----------------------------------------------------
trait IpAddrx {
  fn display(&self);
}

struct V4x(String);
impl IpAddrx for V4x {
  fn display(&self) {
    println!("ipv4 -->>> {:?}", self.0);
  }
}

struct V6x(String);
impl IpAddrx for V6x {
  fn display(&self) {
    println!("ipv6 -->>> {:?}", self.0);
  }
}









