
pub fn comm() {
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

}

