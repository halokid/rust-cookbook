/*
中介者模式
 */

use std::collections::HashMap;

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
    self.colleagues.get(s).unwrap().receive_msg(msg);
  }

  fn get(&self, s:  &String) 
}

struct Colleague(String);