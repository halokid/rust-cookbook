
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(FromPrimitive)]
enum MyEnum {
  A = 1,
  B,
  C,
}

pub fn comm() {
  /*
  // translate the enum to integer, compile pass!
  let x = MyEnum::C as i32;

  match x {
    MyEnum::A => {}
    MyEnum::B => {}
    MyEnum::C => {}
    _ => {}
  }
   */

  let x = 2;

  match FromPrimitive::from_i32(x) {
    Some(MyEnum::A) => println!("Got A"),
    Some(MyEnum::B) => println!("Got B"),
    Some(MyEnum::C) => println!("Got C"),
    None => println!("Counld not convert {}", x),
  }
}









