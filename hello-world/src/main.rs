fn main() {
  println!("Hello, world!");

  /** tuple */
//  let tup: (i32, f64, u8) = (500, 6.4, 1);
//  let (x, y, z) = tup;
//  println!("The value of y is: {}", y);

  let x: (i32, f64, u8) = (500, 6.4, 1);
  let five_hundred = x.0;
  let six_point_four = x.1;
  let one = x.2;
  println!("x.0 is {}", five_hundred);

  /** array */
  let a = [1, 2, 3, 4, 5];

  let b = [3; 5];

//  let index = 10;
//  let elem = a[index];
//  println!("elem: {}", elem);

  let i: i32;
  i = 9;
  let b: bool;
  b = false;
  println!("i: {}, b: {}", i, b);

  let x = five();
  println!("five(): {}", x);

  for number in (1..4).rev() {
    println!("{}!", number);
  }
  println!("LIFTOFF!!!");

  let mut s = String::from("hello");
  s.push_str(", world!");
  println!("{}", s);

  let x = 5;
  let y = x;

  let s1 = String::from("hello");
//  println!("s1 {}", s1);
//  let s2 = s1;
//  println!("s1 {}", s1);        // 错误的引用, s1 已经被移除
//  println!("s2 {}", s2);

//  let (s2, len) = calculate_length(s1);
//  println!("The length of '{}' is {}.", s2, len);

  let lenx = calculate_length_nouse(&s1);
  println!("The lenx is {}.",lenx);
}

// 获取s的所有权
fn calculate_length(s: String) -> (String, usize) {
  let length = s.len();

  (s, length)
}

// 获取s的引用, 不用获取s的所有权
fn calculate_length_nouse(s: &String) -> usize {
  s.len()
}

fn five() -> i32 {
  5
}

fn plus_one(x: i32) -> i32 {
  x + 1
}
