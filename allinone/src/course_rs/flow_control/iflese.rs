



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
      break
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
}








