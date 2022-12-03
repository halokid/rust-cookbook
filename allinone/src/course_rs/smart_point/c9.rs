use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Owner {
  name: String,
  gadgets: RefCell<Vec<Weak<Gadget>>>,   // todo: Owner 与 Gadget 可以一对多
}

struct Gadget {
  id: i32,
  owner: Rc<Owner>,   // todo: gadget 与 owner 可以多对一
}

pub fn comm() {
  // 创建一个 Owner
  // 需要注意，该 Owner 也拥有多个 `gadgets`
  let gadget_owner: Rc<Owner> = Rc::new(
    Owner {
      name: "Gadget Man".to_string(),
      gadgets: RefCell::new(Vec::new()),
    }
  );

  // 创建工具，同时与主人进行关联：创建两个 gadget，他们分别持有 gadget_owner 的一个引用。
  let gadget1 = Rc::new(Gadget { id: 1, owner: gadget_owner.clone() });
  let gadget2 = Rc::new(Gadget { id: 2, owner: gadget_owner.clone() });

  // 为主人更新它所拥有的工具
  // 因为之前使用了 `Rc`，现在必须要使用 `Weak`，否则就会循环引用
  gadget_owner.gadgets.borrow_mut().push(Rc::downgrade(&gadget1));
  gadget_owner.gadgets.borrow_mut().push(Rc::downgrade(&gadget2));

  // 遍历 gadget_owner 的 gadgets 字段
  for gadget_opt in gadget_owner.gadgets.borrow().iter() {

    // gadget_opt 是一个 Weak<Gadget> 。 因为 weak 指针不能保证他所引用的对象
    // 仍然存在。所以我们需要显式的调用 upgrade() 来通过其返回值(Option<_>)来判
    // 断其所指向的对象是否存在。
    // 当然，Option 为 None 的时候这个引用原对象就不存在了。
    let gadget = gadget_opt.upgrade().unwrap();
    println!("Gadget {} owned by {}", gadget.id, gadget.owner.name);
  }

  // 在 main 函数的最后，gadget_owner，gadget1 和 gadget2 都被销毁。
  // 具体是，因为这几个结构体之间没有了强引用（`Rc<T>`），所以，当他们销毁的时候。
  // 首先 gadget2 和 gadget1 被销毁。
  // 然后因为 gadget_owner 的引用数量为 0，所以这个对象可以被销毁了。
  // 循环引用问题也就避免了
}



















