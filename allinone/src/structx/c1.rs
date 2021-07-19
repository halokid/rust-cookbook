/*
struct的构造函数
 */

struct Person<N, A> {
  name: N,
  age: A,
}

// ------ 定义 trait ----------
trait Name {
  fn get_name(&self) -> String {    // todo: 要让impl类能够引用， 就要加 &self 句柄
    "halokid".to_string()
  }
}

trait Age {
  fn get_age(&self) -> i32 {
    18
  }
}

// ------ 要使用trait，需要定义实现的struct ----------
struct Namex;
impl Name for Namex {}
impl Namex {
  fn get_name_from_struct(&self) -> String {
    self.get_name()
  }
}

struct Agex;
impl Age for Agex {}
impl Agex {
  fn get_age_from_struct(&self) -> i32 {
    self.get_age()
  }
}

impl<N: Name, A: Age> Person<N, A> {
  // todo: 这里用String是错的， 因为泛型是一个声明， 所以这里只能用 trait
  fn new(name: N, age: A) -> Self {
    Person {
      name: name,
      age: age
    }
  }
}

pub fn comm() {
  let name = Namex{};
  let age = Agex{};
  let person = Person::new(name, age);
  let name_from_trait = person.name.get_name();
  println!("name_from_trait --------- {}", name_from_trait);

  let age_from_trait = person.age.get_age();
  println!("age_from_trait ----------- {}", age_from_trait);
}





