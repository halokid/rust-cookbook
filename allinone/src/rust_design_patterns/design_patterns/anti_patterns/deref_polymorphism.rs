
// todo: example similar as below java code, implement polymorphism
/*
class Foo {
    void m() { ... }
}

class Bar extends Foo {}

public static void main(String[] args) {
    Bar b = new Bar();
    b.m();
}
 */

use std::ops::Deref;

struct Foo {}

impl Foo {
  fn m(&self) {
    println!("i am Foo.m() use Deref with Bar");
  }
}

struct Bar {
  f: Foo,
}

impl Deref for Bar {
  type Target = Foo;

  fn deref(&self) -> &Self::Target {
    &self.f
  }
}

impl Bar {
  fn get_name(&self) -> &str {
    "i am Bar.get_name()"
  }
}

pub fn comm() {
  let b = Bar {
    f: Foo {}
  };
  b.m();

  b.get_name();
}




