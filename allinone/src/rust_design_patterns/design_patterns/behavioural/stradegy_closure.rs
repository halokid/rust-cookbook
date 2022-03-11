
#[derive(Debug)]
struct Adder;

impl Adder {
  pub fn add<F> (x: u8, y: u8, f: F) -> u8
  where
      F:  Fn(u8, u8) -> u8,
  {
    f(x, y)
  }
}

pub fn comm() {
  let arith_adder = |x, y| x + y;
  // println!("arith_addr -->>> {:#?}", arith_addr);

  let bool_adder = |x, y| {
    if x == 1 || y == 1 {
      1
    } else {
      0
    }
  };
  // println!("bool_addr -->>> {:#?}", bool_addr);

  let custom_adder = |x, y| 2 * x + y;
  // println!("custom_addr -->>> {:#?}", custom_addr);

  assert_eq!(9, Adder::add(4, 5, arith_adder));
  assert_eq!(0, Adder::add(0, 0, bool_adder));
  assert_eq!(5, Adder::add(1, 3, custom_adder));
  // assert_eq!(15, Adder::add(1, 3, custom_adder));

  // todo: option in rust already has the strategy impl in closure!
  let val = Some("Rust");

  let len_strtegy = |s: &str| s.len();
  // todo: the `Option + map()` it means Option-Some-val is the parameter of the map(fn), the same as `fn(Some(val))` the return is a Option
  println!("len_strategy -->>> {}", val.map(len_strtegy).unwrap());

  let first_byte_strategy = |s: &str| s.bytes().next().unwrap();
  println!("first_byte_strategy -->>> {}", val.map(first_byte_strategy).unwrap());
}




