
// loop
pub fn comm() {
  let mut i = 1;
  let something = loop {
    i *= 2;
    if i > 100 {
      break i;        // todo: 跳出 loop， 然后返回 i
    }
  };
  assert_eq!(something, 128);
}

// while
pub fn comm2() {
  let mut counter = 0;

  while counter < 10 {
    println!("hello");
    counter = counter + 1;
  }
}

// for 迭代器
pub fn comm3() {
  let a = [10, 20, 30, 40, 50];

  for elem in a.iter() {
    println!("The value is: {}", elem);
  }

  // 范围创建迭代器, 这个例子， 循环包含0， 不包含5
  for item in 0..5 {
    println!("{}", item * 2);
  }
}




