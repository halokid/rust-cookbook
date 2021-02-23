/*
代理模式
 */

// todo: 1. 创建一个接口。
trait Subject {
  fn get_something(&mut self) -> usize;
}

// todo: 2. 创建实现接口的实体类。
struct RealSubject(usize);
impl RealSubject {
  fn new() -> RealSubject {
    let mut rs = RealSubject(0);
    rs.load_something();
    rs
  }

  fn load_something(&mut self) {
    println!("Try to load something, it is extremely heavy");
    self.0 = 39;      // struct的第一个属性
  }
}

impl Subject for RealSubject {
  fn get_something(&mut self) -> usize {
    self.0
  }
}

// todo: 3. 当被请求时，使用 Proxy 来获取 RealSubject 类的对象。
struct Proxy(Option<RealSubject>);
impl Proxy {
  fn new() -> Proxy {
    Proxy(None)
  }
}

impl Subject for Proxy {
  fn get_something(&mut self) -> usize {
    match self.0 {
      Some(ref mut something) => {
        something.get_something()
      },

      None => {
        let mut rs = RealSubject::new();
        let x = rs.get_something();
        self.0 = Some(rs);
        x
      }
    }
  }
}


fn main() {
  let mut rs = RealSubject::new();
  println!("建立真正的执行类");
  println!("{}", rs.get_something());
  println!("------------------------------------");
  let mut p1 = Proxy::new();      // todo: 建立代理类
  println!("建立代理类1");
  let mut p2  = Proxy::new();
  println!("建立代理类2");

  println!("{}", p1.get_something());   // todo: 通过调用proxy类的方法去调用真正的处理逻辑
  println!("{}", p2.get_something());
}






