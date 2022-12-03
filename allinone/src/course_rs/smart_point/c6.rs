
use std::cell::{Cell, RefCell};
use std::rc::Rc;

pub fn comm() {
  let c = Cell::new("asdf");
  let one = c.get();
  c.set("qwer");

  let two = c.get();
  println!("one -->>> {}, two -->>> {}", one, two);

  // --------------------------------------------------
  // let s = RefCell::new(String::from("hello, world"));
  // let s1 = s.borrow();
  // let s2 = s.borrow_mut();
  // println!("s1 -->>> {}, s2 -->>> {}", s1, s2);

  // --------------------------------------------------
  let mq = MsgQueue {
    msg_cache: RefCell::new(Vec::new()),
  };
  mq.send("hello, world".to_string());
  println!("mq -->>> {:?}", mq);

  // --------------------------------------------------
  let s = Rc::new(RefCell::new("i like change, has \
  many master".to_string()));

  let s1 = s.clone();
  let s2 = s.clone();
  s2.borrow_mut().push_str(", on yeah!");

  println!("s -->>> {:?},\ns1 -->>> {:?},\ns2 -->>> {:?}\n", s, s1, s2);
}

/*
pub trait Messager {
  fn send(&self, msg: String);
}

struct MsgQueue {
  msg_cache:  Vec<String>,
}

// todo: cannot borrow `self.msg_cache` as mutable, as it is behind a `&` reference
impl Messager for MsgQueue {
  fn send(&self, msg: String) {
    self.msg_cache.push(msg);
  }
}
 */

// -------------------------------------------------
pub trait Messager {
  fn send(&self, msg: String);
}

#[derive(Debug)]
pub struct MsgQueue {
  msg_cache: RefCell<Vec<String>>,
}

impl Messager for MsgQueue {
  fn send(&self, msg: String) {
    self.msg_cache.borrow_mut().push(msg);
  }
}











