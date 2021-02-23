/*
外观模式
 */

fn worker_a() {
  println!("hoge");
}

fn worker_b() {
  println!("huga");
}

fn worker_c() {
  println!("piyo");
}

struct Facade;
impl Facade {
  // todo: 统一对外调用的简单的方法, 不用关心 worker_a等方法实现
  fn facade_method(&self) {
    worker_a();
    worker_b();
    worker_c();
  }
}

fn main() {
  let f = Facade;
  f.facade_method();
}