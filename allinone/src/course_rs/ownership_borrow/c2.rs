


pub fn comm() {
  let x = 5;
  let y = &x;     // reference

  assert_eq!(5, x);
  // assert_eq!(5, y);     // todo: wrong, the type if different
  assert_eq!(5, *y);     // dereference

  let mut s = String::from("hello");
  change(&mut s);
  println!("s -->>> {}", s);

  let mut sx = String::from("double mut reference");

  // todo: one scopr just can only borrow mut reference varible once, util drop it.
  {
    // todo: different scope
    let r1 = &mut sx;
    // let r2 = &mut sx;
    // println!("{}, {}", r1, r2);
    println!("{}", r1);
  }     // todo: program drop r1

  let r2 = &mut sx;
  println!("{}", r2);

  // println!("{}", r1);

  comm2();
}

fn change(some_string: &mut String) {
  some_string.push_str(", world");
}


pub fn comm2() {
   let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // 新编译器中，r1,r2作用域在这里结束

    let r3 = &mut s;
    println!("{}", r3);
} // 老编译器中，r1、r2、r3作用域在这里结束
  // 新编译器中，r3作用域在这里结束




