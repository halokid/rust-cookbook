
use typename::TypeName;

#[test]
fn test_get_type() {
  assert_eq!(String::type_name(), "std::string::String");
  assert_eq!(Vec::<i32>::type_name(), "std::vec::Vec<i32>");
  assert_eq!([0, 1, 2].type_name_of(), "[i32; 3]");

  let a = 65u8;
  let b = b'A';
  let c = 65;
  let d = 65i8;
  let e = 65i32;
  let f = 65u32;

  let arr = [1,2,3,4,5];
  let first = arr[0];

  println!("type of a 65u8  {} is {}", a, a.type_name_of());
  println!("type of b b'A'  {} is {}", b, b.type_name_of());
  println!("type of c 65    {} is {}", c, c.type_name_of());
  println!("type of d 65i8  {} is {}", d, d.type_name_of());
  println!("type of e 65i32 {} is {}", e, e.type_name_of());
  println!("type of f 65u32 {} is {}", f, f.type_name_of());

  println!("type of arr {:?} is {}", arr, arr.type_name_of());
  println!("type of first {} is {}", first, first.type_name_of());
}