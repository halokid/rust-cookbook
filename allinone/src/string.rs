
pub fn comm() {
  /*
  let s1 = String::from("hello, ");
  let s2 = String::from("world");

  let s4 = format!("{}-{}", s1, s2);

  let s3 = s1 + &s2;        // todo: 这里 s1, s2 已经用过了
  println!("s3: {}", s3);
  // let s4 = format!("{}-{}-{}", s1, s2, s3);
  println!("s4: {}", s4);

  let v = String::from("hello");
  for c in v.chars(){
    println!("{}", c);
  }
   */


  let a = "hello world";      // 静态内存
  let b = "OK";               // 静态内存
  let mut s = String::from("Hello Rust");
  println!("s capacity: {}", s.capacity());

  s.push_str("Here I come!");
  println!("s.len {}", s.len());

  let s = "Hello Rust!";
  println!("{}", s.len());

  // &str => String
  let c = a.to_string();
  let d = String::from(b);
  let d = a.to_owned();

  // String => &str
  let e = &String::from("Hello Rust");
  let ex = e.as_str();
  // 不能直接这样使用
  // let e = String::from("Hello Rust").as_str();

  // String + &str => String
  // String后面接上N个&str
  let mut strs = "Hello".to_string();
  strs.push_str( " Rust");
  println!("strs: {}", strs);

  let x= "xyz";     // todo: 判断字符串是否相等
  if x == "xyz" {
    println!("xyz hit");
  } else {
    println!("xyz no!!!");
  }

  let xx = String::from("xyz");
  if xx == "xyz" {
    println!("xx hit");
  } else {
    println!("xx no!!!");
  }
}







