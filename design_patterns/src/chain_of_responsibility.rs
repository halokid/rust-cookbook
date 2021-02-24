/*
责任链模式
 */

// todo: 1. 创建责任链接口
trait Cor {
  fn process_request(&self, _: &mut dyn Request);   // todo: 参数是接收一个实体处理类的实例
}

// todo: 2. 创建责任链实际调用的逻辑处理实体
trait Request {       // todo: 定时实际处理时的接口
  fn get_level(&self) -> Level;
  fn get_something(&self) -> usize;
}

struct RequestX {
  level:    Level,
  v:        usize,
}

impl RequestX {
  fn new(l:  Level,  v: usize) -> RequestX {
    RequestX {
      level:    l,
      v:        v,
    }
  }
}

impl Request for RequestX {
  fn get_level(&self) -> Level {
    self.level
  }

  fn get_something(&self) -> usize {
    self.v
  }
}

// todo: 3. 定义责任链的链节点, 也可以理解为不同的处理级别等
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Level {
  High,
  Middle,
  Low,
}

// todo: 4. 责任链实体类
struct ImplCor {
  next:    Option<Box<dyn Cor>>,
  allowable_level:    Level,
}

impl ImplCor {
  fn new(l: Level, next: Option<Box<dyn Cor>>) -> ImplCor {
    ImplCor {
      next:     next,
      allowable_level:   l,
    }
  }
}

impl Cor for ImplCor {
  fn process_request(&self, r: &mut dyn Request) {
    println!("level: {:?} --- ", self.allowable_level);
    if self.allowable_level == r.get_level() {
      println!("Requet accepted -v = {}", r.get_something());
    } else {

      if let Some(ref next) = self.next {
        println!("Pass to the next");
        next.process_request(r);
      }
      else {
        println!("Chain finished!");
      }
    }
  }
}

fn main() {
  let high = ImplCor::new(Level::High, None);
  let middle = ImplCor::new(Level::Middle, Some(Box::new(high)));
  let low = ImplCor::new(Level::Low, Some(Box::new(middle)));

  let mut r1 = RequestX::new(Level::High, 1);
  let mut r2 = RequestX::new(Level::Middle, 2);
  let mut r3 = RequestX::new(Level::Low, 3);

  low.process_request(&mut r3);
  // low.process_request(&mut r2);
  // low.process_request(&mut r1);
}





