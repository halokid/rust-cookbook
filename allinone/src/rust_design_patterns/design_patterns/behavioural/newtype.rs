
struct Foo {

}

impl Foo {

}

pub struct Bar(Foo);

impl Bar {
  pub fn new() -> Self {
    Bar{ 0: Foo {} }
  }
}

pub fn comm() {
  let b = Bar::new();
  let f = b.0;    // this is a new wrap type
}