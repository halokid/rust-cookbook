pub fn comm() {
  let s = String::from("hello,world!");
  say_hello(&s);
  say_hello(&s[..]);
  say_hello(s.as_str());

  // ----------------------------------------
  let mut s = String::from("Hello ");
  s.push('r');
  println!("追加字符 push() -> {}", s);

  s.push_str("ust!");
  println!("追加字符串 push_str() -> {}", s);

  let mut s = String::from("Hello rust!");
  s.insert(5, ',');
  println!("插入字符 insert() -> {}", s);
  s.insert_str(6, " I like");
  println!("插入字符串 insert_str() -> {}", s);

  // ------------------------------------------
  let string_replace = String::from("I like rust. Learning rust is my favorite!");
  let new_string_replace = string_replace.replace("rust", "RUST");
  dbg!(new_string_replace);

  // ------------------------------------------
  let string_replace = "I like rust. Learning rust is my favorite!";
  let new_string_replacen = string_replace.replacen("rust", "RUST", 1);
  dbg!(new_string_replacen);

  // ------------------------------------------
  let mut string_replace_range = String::from("I like rust!");
  string_replace_range.replace_range(7..8, "R");
  dbg!(string_replace_range);

  // ------------------------------------------
  let mut string_remove = String::from("测试remove方法");
  println!(
    "string_remove 占 {} 个字节",
    std::mem::size_of_val(string_remove.as_str())
  );
  // 删除第一个汉字
  string_remove.remove(0);
  // 下面代码会发生错误
  // string_remove.remove(1);
  // 直接删除第二个汉字
  string_remove.remove(3);
  dbg!(string_remove);

  // -----------------------------------------
  let byte_escape = "I'm writing \x52\x75\x73\x74!";
  println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

  // \u 可以输出一个 unicode 字符
  let unicode_codepoint = "\u{211D}";
  let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

  println!(
    "Unicode character {} (U+211D) is called {}",
    unicode_codepoint, character_name
  );

  // 换行了也会保持之前的字符串格式
  let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here ->\
                        <- can be escaped too!";
  println!("{}", long_string);

  println!("{}", "hello \\x52\\x75\\x73\\x74");
  let raw_str = r"Escapes don't work here: \x3F \u{211D}";
  println!("{}", raw_str);

  // 如果字符串包含双引号，可以在开头和结尾加 #
  let quotes = r#"And then I said: "There is no escape!""#;
  println!("{}", quotes);

  // 如果还是有歧义，可以继续增加，没有限制
  let longer_delimiter = r###"A string with "# in it. And even "##!"###;
  println!("{}", longer_delimiter);

  // ------------------------------------------
  for c in "你好吗".chars() {
    println!("{}", c);
  }

  for b in "你好吗".bytes() {
    println!("{}", b);
  }

  // -------------------------------------------
  {
    let s = String::from("hello"); // 从此处起，s 是有效的
    // 使用 s
  }                                  // 此作用域已结束，
  // s 不再有效，内存被释放
}

fn say_hello(s: &str) {
  println!("{}", s);
}




