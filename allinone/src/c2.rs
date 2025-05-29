use std::ops::Deref;

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    // &self.0
    println!("-->>> call deref here");
    &self.0
  }
}

fn hello(name: &str) {
  println!("Hello, {}!", name);
}

fn main() {
  let my_box = MyBox(String::from("Rust"));
  hello(&my_box); // TODO: `&my_box` will auto call `deref` fn
}

