use std::cell::RefCell;
use std::rc::Rc;
use crate::course_rs::smart_point::c8::List::{Cons, Nil};

#[derive(Debug)]
enum List {
  Cons(i32, RefCell<Rc<List>>),
  Nil,
}

impl List {
  fn tail(&self) -> Option<&RefCell<Rc<List>>> {
    match self {
      List::Cons(_, item) => Some(item),
      List::Nil => None,
    }
  }
}

pub fn comm() {
  let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

  println!("a initial rc count is -->>> {}", Rc::strong_count(&a));
  println!("a point to node -->>> {:?}", a.tail());

  // create `b` to `a` reference
  let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

  println!("after created `b`, `a` rf count -->>> {}", Rc::strong_count(&a));
  println!("b initial rc count is -->>> {}", Rc::strong_count(&b));
  println!("b point to node -->>> {:?}", b.tail());

  // todo: use the mutable of RefCell, create the reference from `a` to `b`
  if let Some(link) = a.tail() {
    *link.borrow_mut() = Rc::clone(&b);
  }

  println!("after change `a`, `b` rc count -->>> {}", Rc::strong_count(&b));
  println!("after change `a`, `a` rc count -->>> {}", Rc::strong_count(&a));

  // 下面一行println!将导致循环引用
  // 我们可怜的8MB大小的main线程栈空间将被它冲垮，最终造成栈溢出
  // println!("a next item = {:?}", a.tail());

  // todo: weak solve self-circulation problem ----------------------
  let five = Rc::new(5);

  // todo: use Rc::downgrade create a Weak point, Rc to Weak
  let weak_five = Rc::downgrade(&five);

  // todo: use Wek::upgrade create a Rc, Weak to Rc
  let strong_five: Option<Rc<_>> = weak_five.upgrade();
  println!("strong_five 1 -->>> {}", strong_five.unwrap());

  // release `five` manual
  drop(five);

  // Wek reference resource not exist, so return None
  let strong_five: Option<Rc<_>> = weak_five.upgrade();
  println!("strong_five 2 -->>> {:?}", strong_five);
}





