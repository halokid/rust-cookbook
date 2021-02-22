/*
配适器模型
 */

trait Adapter {
  fn get_a(&self) -> usize;
  fn get_b(&self) -> usize;
}

struct ObjectX {
  a:  usize,
  b:  usize,
}

impl Adapter for ObjectX {
  fn get_a(&self) -> usize {
    self.a
  }

  fn get_b(&self) -> usize {
    self.b
  }
}

// -------------------------------------------
struct ObjectY {
  m:   u8,
  n:   u8,
}

impl Adapter for ObjectY {
  fn get_a(&self) -> usize {
    self.m as usize
  }

  fn get_b(&self) -> usize {
    self.n as usize
  }
}

// ------------------------------------------
fn main() {
  let obj_x = ObjectX{ a: 10,  b: 120};
  let obj_y = ObjectY{ m: 1, n: 2};

  use_via_adapter(&obj_x);
  use_via_adapter(&obj_y);
}

// todo: 通往各个不兼容接口的桥梁
fn use_via_adapter(adapter:  &dyn Adapter) {
  println!("通过adapter来调度object, a = {}, b = {}", adapter.get_a(), adapter.get_b());
}






