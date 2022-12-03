use std::rc::Rc;

struct Owner {
  name: String,
}

struct Gadget {
  id: i32,
  owner: Rc<Owner>,
}

pub fn comm() {
  // create a `Owner` base on reference counter
  let gadget_owner: Rc<Owner> = Rc::new(Owner {
    name: "Gadget Man".to_string(),
  });

  // create two tools, they are belong the same owner
  let gadget1 = Gadget {
    id: 1,
    owner: Rc::clone(&gadget_owner),
  };

  let gadget2 = Gadget {
    id: 2,
    owner: Rc::clone(&gadget_owner),
  };

  // release first `Rc<Owner>`
  drop(gadget_owner);

  // 尽管在上面我们释放了 gadget_owner，但是依然可以在这里使用 owner 的信息
  // 原因是在 drop 之前，存在三个指向 Gadget Man 的智能指针引用，上面仅仅
  // drop 掉其中一个智能指针引用，而不是 drop 掉 owner 数据，外面还有两个
  // 引用指向底层的 owner 数据，引用计数尚未清零
  // 因此 owner 数据依然可以被使用
  println!("Gadget {} owned by {}", gadget1.id, gadget1.owner.name);
  println!("Gadget {} owned by {}", gadget2.id, gadget2.owner.name);

  // 在函数最后，`gadget1` 和 `gadget2` 也被释放，最终引用计数归零，随后底层
  // 数据也被清理释放
}








