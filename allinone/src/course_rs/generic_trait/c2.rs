use std::fmt::Display;

pub trait Summary {
  // fn summarize(&self) -> String;

  fn summarize(&self) -> String {
    String::from("(Read more...)")
  }
}

pub struct Post {
  pub title: String,
  pub author: String,
  pub content: String,
}

impl Summary for Post {

  // todo: if `POST` no rewrite the trait fn, just use the origin
  // fn summarize(&self) -> String {
    // format!("Article {}, Author {}", self.title, self.author)
  // }
}

pub struct Weibo {
  pub username: String,
  pub content:  String,
}

impl Summary for Weibo {
  fn summarize(&self) -> String {
    format!("{} send a weibo {}", self.username, self.content)
  }
}

// todo: this is syntactic sugar
pub fn notify(item: &impl Summary) {
  println!("Breanking news! {}", item.summarize());
}

// todo: real world use
pub fn notify_x<T: Summary>(item: &T) {
  println!("Breanking news! {}", item.summarize());
}

pub fn notify_y<T: Summary + Display>(item: &T) {
  println!("Breanking news! {}", item);
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
  let mut largest = list[0];

  for &item in list.iter() {
    if item > largest {
      largest = item;
    }
  }
  largest
}

pub fn comm() {
  let p = Post {
    title: "my post".to_string(),
    author: "halokid".to_string(),
    content: "my content".to_string()
  };
  println!("{}", p.summarize());;

  let w = Weibo {
    username: "halokid".to_string(),
    content: "my weibo".to_string()
  };
  println!("{}", w.summarize());;

  // ----------------------------------------
  let number_list = vec![34, 50, 76, 98, 56];

  let result = largest(&number_list);
  println!("the largest number is {}", result);

  let char_list = vec!['y', 'v', 'k', 'm'];
  let result = largest(&char_list);
  println!("the largest char is {}", result);
}










