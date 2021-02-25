/*
中介者模式
 */

use std::collections::HashMap;

// todo: 1. 创建中介类。
struct Mediator {
  colleagues:     HashMap<String, Colleague>,
}

impl Mediator {
  fn new() -> Mediator {
    Mediator {
      colleagues:     HashMap::new(),
    }
  }

  fn add_colleague(&mut self, c:  Colleague) {
    self.colleagues.insert(c.0.clone(), c);
  }

  fn consult_to(&self, s: &String, msg: String) {
    // todo: 中介者调用其他的类去  receive_mgg 数据
    self.colleagues.get(s).unwrap().receive_msg(msg);
  }

  fn get(&self, s:  &String) -> &Colleague {
    self.colleagues.get(s).unwrap()
  }
}

// todo: 创建具体执行的实体类
struct Colleague(String);
impl Colleague {
  fn new(s: &String) -> Colleague {
    Colleague(s.clone())
  }

  // todo: 执行的实际动作, 实际上是调用 中介者的实体，然后执行中介者的方法
  fn send_msg(&self, m: &Mediator, to: &String, msg: String) {
    m.consult_to(to, msg);
  }

  fn receive_msg(&self, msg: String) {
    println!("{} gets {}", self.0, msg);
  }
}

fn main() {
  let mut mediator = Mediator::new();
  let key1 = "Halokid".to_string();
  let c1 = Colleague::new(&key1);

  let key2 = "Piyo".to_string();
  let c2 = Colleague::new(&key2);

  mediator.add_colleague(c1);
  mediator.add_colleague(c2);

  let c1 = mediator.get(&key1);
  c1.send_msg(&mediator, &key2, "hi form Halokid".to_string());

  let c2 = mediator.get(&key2);
  c2.send_msg(&mediator, &key1, "hi for Piyo".to_string());
}









