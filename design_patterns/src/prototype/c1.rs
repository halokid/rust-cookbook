/*
原型模式
 */

trait Prototype: Clone {
  fn set_x(&mut self, _: usize);
  fn set_y(&mut self, _: usize);
}

#[derive(Debug, Clone)]
struct Object {
  x:  usize,
  y:  usize,
}

impl Object {
  fn new() -> Object {
    Object {
      x:    100,
      y:    200,
    }
  }
}

impl Prototype for Object {
  fn set_x(&mut self, x: usize) {
    self.x = x
  }

  fn set_y(&mut self, y: usize) {
    self.y = y
  }
}

fn main() {
  let origin = Object::new();
  // let origin_addr = origin as *const i32;
  // let origin_addr = &origin;
  println!("origin ptr addr: {:p}", &origin);

  let mut obj = origin.clone();
  obj.set_x(39);
  println!("obj ptr addr: {:p}", &obj);

  println!("origin = {:?}", origin);
  println!("obj = {:?}", obj);
}





