use tokio::select;

pub fn comm() {
  // todo: 在栈上创建一个长度为1000的数组
  let arr = [0; 1000];
  // todo: 将arr所有权转移arr1，由于 `arr` 分配在栈上，因此这里实际上是直接重新深拷贝了一份数据
  let arr1 = arr;

  // arr 和 arr1 都拥有各自的栈上数组，因此不会报错
  println!("{:?}", arr.len());
  println!("{:?}", arr1.len());

  // todo: 在堆上创建一个长度为1000的数组，然后使用一个智能指针指向它
  let arr = Box::new([0; 1000]);
  // todo: 将堆上数组的所有权转移给 arr1，由于数据在堆上，因此仅仅拷贝了智能指针的结构体，底层数据并没有被拷贝
  // todo: 所有权顺利转移给 arr1，arr 不再拥有所有权
  let arr1 = arr;
  println!("{:?}", arr1.len());
  // 由于 arr 不再拥有底层数组的所有权，因此下面代码将报错
  // println!("{:?}", arr.len());

  // ------------------------------------------------------
  let elems: Vec<Box<dyn Draw>> = vec![Box::new(Button{id: 1}),
                                      Box::new(Select{id: 2})];
  for e in elems {
    e.draw();
  }

  // -------------------------------------------------------
  // todo: tow Deref of Box<>
  let arr = vec![Box::new(1), Box::new(2)];
  let (first, second) = (&arr[0], &arr[1]);
  let sum = **first + **second;
  println!("sum -->>> {}", sum);

  // ------------------------------------------------------
  let x = 5;
  let y = &x;
  println!("x -->>> {}", x);
  println!("y -->>> {:?}", y);
  assert_eq!(5, *y);
  assert_eq!(6, *y);
}

// todo: 在 Rust 中，想实现不同类型组成的数组只有两个办法：枚举和特征对象，前者限制较多，因此后者往往是最常用的解决办法。
trait Draw {
  fn draw(&self);
}

struct Button {
  id: u32,
}

impl Draw for Button {
  fn draw(&self) {
    println!("this is sreen which {} number button", self.id);
  }
}

struct Select {
  id: u32,
}

impl Draw for Select {
  fn draw(&self) {
    println!("this button is hard to use {}", self.id);
  }
}



