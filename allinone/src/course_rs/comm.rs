pub fn comm() {
  println!("Hello");                 // => "Hello"
  println!("Hello, {}!", "world");   // => "Hello, world!"
  println!("The number is {}", 1);   // => "The number is 1"
  println!("{:?}", (3, 4));          // => "(3, 4)"
  println!("{value}", value = 4);      // => "4"
  println!("{} {}", 1, 2);           // => "1 2"
  println!("{:04}", 42);             // => "0042" with leading zeros

  let s = "hello";
  println!("{}, world", s);
  let s1 = format!("{}, world", s);
  print!("{}", s1);
  print!("{}\n", "!");

  eprintln!("Error: Could not complete task");

  // ----------------------------------------------------------
  let i = 3.1415926;
  let s = String::from("hello");
  let v = vec![1, 2, 3];
  let p = Person { name: "sunface".to_string(), age: 18 };
  println!("{:?}, {:?}, {:?}, {:?}", i, s, v, p);

  // ---------------------------------------------------------
  let i = 3.1415926;
  let s = String::from("hello");
  let v = vec![1, 2, 3];
  let p = Person {
    name: "sunface".to_string(),
    age: 18,
  };
  println!("{}, {}, {:#?}, {:#?}", i, s, v, p);

  // ---------------------------------------------------------
  let p = Person {
    name: "sunface".to_string(),
    age: 18,
  };
  println!("{}", p);

  // ----------------------------------------------------------
  let arr = Array(vec![1, 2, 3]);
  println!("{}", arr);

  // ---------------------------------------------------------
  println!("{}{}", 1, 2); // =>"12"
  println!("{1}{0}", 1, 2); // =>"21"
  // => Alice, this is Bob. Bob, this is Alice
  println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
  println!("{1}{}{0}{}", 1, 2); // => 2112

  // --------------------------------------------------------
  let v = 3.1415926;
  // Display => 3.14
  println!("{:.2}", v);
  // Debug => 3.14
  println!("{:.4?}", v);

  //-----------------------------------
  // 以下全部输出 "Hello x    !"
  // 为"x"后面填充空格，补齐宽度5
  println!("Hello {:5}!", "x");
  // 使用参数5来指定宽度
  println!("Hello {:1$}!", "x", 5);
  // 使用x作为占位符输出内容，同时使用5作为宽度
  println!("Hello {1:0$}!", 5, "x");
  // 使用有名称的参数作为宽度
  println!("Hello {:width$}!", "x", width = 5);
  //-----------------------------------

  // 使用参数5为参数x指定宽度，同时在结尾输出参数5 => Hello x    !5
  println!("Hello {:1$}!{}", "x", 5);

  // --------------------------------------------------
  let v = 3.1415926;
  // 保留小数点后两位 => 3.14
  println!("{:.2}", v);
  // 带符号保留小数点后两位 => +3.14
  println!("{:+.2}", v);
  // 不带小数 => 3
  println!("{:.0}", v);
  // 通过参数来设定精度 => 3.1416，相当于{:.4}
  println!("{:.1$}", v, 4);

  let s = "hi我是Sunface孙飞";
  // 保留字符串前三个字符 => hi我
  println!("{:.3}", s);
  // {:.*}接收两个参数，第一个是精度，第二个是被格式化的值 => Hello abc!
  println!("Hello {:.*}!", 3, "abcdefg");

  // -----------------------------------------------------
  // 二进制 => 0b11011!
  println!("{:#b}!", 27);
  // 八进制 => 0o33!
  println!("{:#o}!", 27);
  // 十进制 => 27!
  println!("{}!", 27);
  // 小写十六进制 => 0x1b!
  println!("{:#x}!", 27);
  // 大写十六进制 => 0x1B!
  println!("{:#X}!", 27);

  // 不带前缀的十六进制 => 1b!
  println!("{:x}!", 27);

  // 使用0填充二进制，宽度为10 => 0b00011011!
  println!("{:#010b}!", 27);

  println!("{:2e}", 1000000000); // => 1e9
  println!("{:2E}", 1000000000); // => 1E9

  let v = vec![1, 2, 3];
  println!("{:p}", v.as_ptr()); // => 0x600002324050

  // {使用{转义，}使用} => Hello {}
  println!("Hello {{}}");
}

// todo: in rust, you can not direct implement trait for external-type, but can
// todo: use newtype to solve this problem
struct Array(Vec<i32>);

impl fmt::Display for Array {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "The array is: {:?}", self.0)
  }
}

#[derive(Debug)]
struct Person {
  name: String,
  age: u8,
}

use std::fmt;
use std::fmt::Formatter;
use hmac::digest::generic_array::arr;

impl fmt::Display for Person {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "this is Display implement for Person, name: {}, age: {}",
      self.name, self.age
    )
  }
}











