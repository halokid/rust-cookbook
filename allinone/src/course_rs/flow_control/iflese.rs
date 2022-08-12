pub fn comm() {
  let n = 6;

  if n % 4 == 0 {
    println!("number 6 divisible by 4");
  } else if n % 3 == 0 {
    println!("number 6 divisible by 3");
  } else if n % 2 == 0 {
    println!("number 6 divisible by 2");
  } else {
    println!("number 6 is not divisible by 4, 3 or 2");
  }

  // ------------------------------------------
  let mut n = 0;
  loop {
    if n > 5 {
      break;
      // println!("i am not `break`, but i am retuen");
      // return;
    }
    println!("{}", n);
    n += 1;
  }
  println!("i am out!");

  // ------------------------------------------
  let a = [10, 20, 30, 40, 50];

  for element in a.iter() {
    println!("the value is: {}", element);
  }
  println!("-->>> another style...");
  let mut index = 0;
  while index < 5 {
    println!("the value is: {}", a[index]);
    index += 1;
  }

  // -------------------------------------------
  let mut counter = 0;
  let result = loop {
    counter += 1;
    if counter == 10 {
      break counter * 2;
    }
  };
  dbg!(&result);

  // --------------------------------------------
  let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];

  v.iter().filter(|x| matches!(x, MyEnum::Foo));
  dbg!(&v);

  let v = Some(3u8);
  match v {
    Some(3) => println!("three"),
    _ => {}
  }

  if let Some(3) = v {
    println!("three more smart!");
  }

  let age = Some(30);
  println!("在匹配前，age是{:?}", age);
  // todo: same varible name, but the type different
  match age {
    Some(age) => println!("匹配出来的age是{}", age),
    _ => ()
  }
  println!("在匹配后，age是{:?}", age);
}

#[derive(Debug)]
enum MyEnum {
  Foo,
  Bar,
}










