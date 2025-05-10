use std::rc::Rc;

/**
题目 1：生命周期 + trait 返回值
*/
trait Describable {
  fn describe(&self) -> &str;
}

struct Person {
  name:  String,
}

impl Describable for Person {
  fn describe(&self) -> &str {
    &self.name
  }
}

/**
是否能编译？如果报错，问题出在哪？
如何正确指定生命周期，使 trait 返回引用时安全？
*/
fn get_desc<'a> (p: &'a dyn Describable) -> &'a str {
  p.describe()
}

// fn get_desc_a(p: &dyn Describable) -> String {
// fn get_desc_a(p: &dyn Describable) -> &str {
//   p.describe().to_string().as_str()
// }

fn get_desc_b(p: &impl Describable) -> String {
  p.describe().to_string()
}

fn get_desc_c(p: Box<dyn Describable>) -> String {
  p.describe().to_string()
}

pub fn test_ownership_return() {
  let p = Person {
    name: "Alice".to_string(),
  };
  println!("Person name -->>> {}", get_desc(&p));

  // println!("Person name 2 -->>> {}", get_desc_a(&p));
  // println!("Person name 3 -->>> {}", get_desc_b(&p));

  let px = Box::new(p);
  println!("Person name 4 -->>> {}", get_desc_c(px));
}


// -------------------------------------------------
/**
题目 2：Rc<T> 多线程共享
*/
pub fn test_rc_multi_thread_shared() {
  let s = Rc::new(String::from("hello"));

  for _ in 0..3 {
    let s_clone = Rc::clone(&s);
    std::thread::spawn(move || {

    });
  }
}

/**
题目 4：生命周期结合结构体
*/
struct Book<'a> {
  title:  &'a str
}

pub fn test_lifetimes_struct() {
  // let title = String::from("Rust");
  // let book = Book {
  //   title: &title,
  // };

  let mut book = Book {
    title: "",
  };
  let title = "Rust";
  book.title = title;

  println!("{}", book.title);
}





