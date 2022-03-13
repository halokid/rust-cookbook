
#[derive(Debug, PartialEq)]
pub struct Foo {
  bar: String,
}

impl Foo {
  pub fn builder() -> FooBuilder {
    FooBuilder::default()
  }
}

#[derive(Default)]
pub struct FooBuilder {
  bar: String,
}

impl FooBuilder {
  pub fn new() -> FooBuilder {
    FooBuilder {
      bar: String::from("X")
    }
  }

  pub fn name(mut self, bar: String) -> FooBuilder {
    self.bar = bar;
    self
  }

  pub fn build(self) -> Foo {
    Foo {
      bar: self.bar,
    }
  }
}

pub fn comm() {
  // direct create Foo
  let foo = Foo {
    bar: "X".to_string()
  };
  println!("foo direct -->>> {:?}", foo);

  // create Foo use `builder pattern`
  let foo_from_builder: Foo = FooBuilder::new().name("Y".to_string()).build();
  println!("foo from builder -->>> {:?}", foo_from_builder);
}



