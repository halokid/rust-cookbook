pub trait Draw {
  fn draw(&self) -> String;
}

impl Draw for u8 {
  fn draw(&self) -> String {
    format!("u8: {}", *self)
  }
}

impl Draw for f64 {
  fn draw(&self) -> String {
    format!("f64: {}", *self)
  }
}

// 若 T 实现了 Draw 特征， 则调用该函数时传入的 Box<T> 可以被隐式转换成函数参数签名中的 Box<dyn Draw>
fn draw1(x: Box<dyn Draw>) {
  // 由于实现了 Deref 特征，Box 智能指针会自动解引用为它所包裹的值，然后调用该值对应的类型上定义的 `draw` 方法
  let s = x.draw();
  println!("draw1 s -->>> {}", s);
}

fn draw2(x: &dyn Draw) {
  let s = x.draw();
  println!("draw2 s -->>> {}", s);
}

/*
pub struct Screen {
  pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
  pub fn run(&self) {
    for component in self.components.iter() {
      component.draw();
    }
  }
}
 */

// todo: generic
pub struct Screen<T: Draw> {
  pub components: Vec<T>,
}

impl<T> Screen<T>
  where T: Draw {
  pub fn run(&self) {
    for component in self.components.iter() {
      component.draw();
    }
  }
}

pub fn comm() {
  let x = 1.1f64;
  // do_something(&x);
  let y = 8u8;

  // x 和 y 的类型 T 都实现了 `Draw` 特征，因为 Box<T> 可以在函数调用时隐式地被转换为特征对象 Box<dyn Draw>
  // 基于 x 的值创建一个 Box<f64> 类型的智能指针，指针指向的数据被放置在了堆上
  draw1(Box::new(x));
  // 基于 y 的值创建一个 Box<u8> 类型的智能指针
  draw1(Box::new(y));
  draw2(&x);
  draw2(&y);
}







