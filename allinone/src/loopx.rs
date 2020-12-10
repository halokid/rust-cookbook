
pub fn comm() {
  let mut i = 1;

  loop {
    println!("i == {}", i);
    if i >= 10 {
      break
    } else {
      i += 1;
    }
  }

  for i in 1..101 {
    match (i % 3 == 0, i % 5 == 0) {
      (true, true) => println!("fizzbuzz"),
      (true, false) => println!("fizz"),
      (false, true) => println!("buzz"),
      (false, false) => println!("{}", i),
    }
  }
}