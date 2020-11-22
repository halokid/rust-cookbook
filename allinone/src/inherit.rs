/*
继承
 */

// ------ 定义结构，存放数据的 ------
struct Book {
  name:   String
}

// ------ 定义结构的行为 -------
impl Book {
  fn show_book_name(&self) {
    println!("book name is {}", self.name);
  }
}


use std::ops::{Deref, DerefMut};
struct MyBook {
  p:        Book,
  author:   String,
}

impl MyBook {
  fn new() -> MyBook {
    MyBook {
      p: Book {name:  "fish-cook".to_string()},
      author: "fishman".to_string()
    }
  }
}

// todo: 继承的关键设定
impl Deref for MyBook {
  type Target = Book;

  fn deref(&self) -> &Book {
    &self.p
  }
}

pub fn comm() {
  let mut mybook = MyBook::new();
  println!("book name attr: {}", mybook.name);
  println!("book author attr: {}", mybook.author);
  mybook.show_book_name();
}

















