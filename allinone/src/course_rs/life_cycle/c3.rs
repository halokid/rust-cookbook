
#[derive(Debug)]
struct Foo;

impl Foo {
  fn mutate_and_share(&mut self) -> &Self {
    &*self
  }

  fn share(&self) {}
}

pub fn comm() {
  // let mut foo = Foo;
  // let loan = foo.mutate_and_share();
  // foo.share();
  // println!("{:?}", loan);
}
