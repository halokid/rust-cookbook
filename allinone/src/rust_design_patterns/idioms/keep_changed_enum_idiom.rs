use std::mem;

enum MyEnum {
  A { name: String, x: u8 },
  B { name: String }
}

fn a_to_b(e: &mut MyEnum) {
  if let MyEnum::A { name, x } = e {
    *e = MyEnum::B { name: mem::take(name) }
  }
}
