use std::cell::RefCell;
use std::rc::Rc;
use libc::linger;

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Clone)]
struct Node {
  value:  String,

  // todo: if we need to change the property from one struct, maybe it can use RefCell to wrap it
  next:   Link,
}

impl Node {
  fn new(value: String) -> Rc<RefCell<Node>> {
    Rc::new(RefCell::new(
      Node {
        value,
        next: None
      }
    ))
  }
}

#[derive(Clone)]
pub struct TransactionLog {
  head:   Link,
  tail:   Link,
  pub length:   u64,
}

impl TransactionLog {
  pub fn new_empty() -> TransactionLog {
    TransactionLog {
      head: None,
      tail: None,
      length: 0
    }
  }

  pub fn append(&mut self, value: String) {
    let new = Node::new(value);

    // todo: take method can get real-value from Option
    match self.tail.take() {
      None => self.head = Some(new.clone()),
      Some(old) => {
        old.borrow_mut().next = Some(new.clone())
      }
    };
    self.length += 1;
    self.tail = Some(new);
  }

  pub fn pop(&mut self) -> Option<String> {
    self.head.take().map(|head| {
      if let Some(next) = head.borrow_mut().next.take() {
        self.head = Some(next)
      } else {
        self.tail.take();
      }

      self.length -= 1;
      Rc::try_unwrap(head).ok().expect("Something is wrong")
        .into_inner().value
    })
  }
}












