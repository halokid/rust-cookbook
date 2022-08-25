use std::convert::TryInto;

pub fn comm() {
  // let a: u8 = 10;
  // let b: u16 = 1500;

  let b: i16 = 1500;

  let b_: u8 = match b.try_into() {
    Ok(b1) => b1,
    Err(e) => {
      println!("{:?}", e.to_string());
      0
    }
  };
}

struct Foo {
  x:  u32,
  y:  u16,
}

struct Bar {
  a:  u32,
  b:  u16,
}

fn reinterpret(foo: Foo) -> Bar {
  let Foo {x, y} = foo;
  Bar {
    a: x,
    b: y,
  }
}