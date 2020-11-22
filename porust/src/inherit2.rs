/*
继承
 */

trait Speaks {   // todo: 实际上是把Speaks独立抽出来了，以后也可以作为 Human 的speaks封装
  fn speak(&self);
}

trait Animal {
  fn animal_type(&self) -> &str;      // 属性1, 用函数来返回
  fn noise(&self) -> &str;            // 属性2
}

// /*
impl<T> Speaks for T where T: Animal {      // todo: 让具备Animal特性的struct 也具备Speaks特性
  // todo: 让 Animal 也能使用 Speaks 的特性
  // todo: 或者说可以连结 Speaks 和 Animal 的所有资源
  fn speak(&self) {
    println!("the {} said {}", self.animal_type(), self.noise());
  }
}
 // */

// impl<T: Animal> Speaks for T {
//   fn speak(&self) {
//     println!("the {} said {}", self.animal_type(), self.noise());
//   }
// }

// impl<T> Speaks for T where T: Animal {
//   fn speak(&self) {
//     println!("The {} said {}", self.animal_type(), self.noise());
//   }
// }


struct Dog {}
struct Cat {}

impl Animal for Dog {
  fn animal_type(&self) -> &str {
    "dog"
  }

  fn noise(&self) -> &str {
    "woof"
  }
}

// ------------------------------------

impl Animal for Cat {
  fn animal_type(&self) -> &str {
    "cat"
  }

  fn noise(&self) -> &str {
    "meow"
  }
}

trait Human {
  fn name(&self) -> &str;
  fn sentence(&self) -> &str;
}

// impl<T> Speaks for T where T: Human {
//   fn speak(&self) {
//     println!("person {} said {}", self.name(), self.sentence());
//   }
// }

// impl<T: Human> Speaks for T where T: Human {
//   fn speak(&self) {
//     println!("person {} said {}", self.name(), self.sentence());
//   }
// }

// /*

impl<T> Speaks for T where T: Human {
  fn speak(&self) {
    println!("{} said {}", self.name(), self.sentence());
  }
}

struct Person {}

impl Human for Person {
  fn name(&self) -> &str {
    "i am Person"
  }

  fn sentence(&self) -> &str {
    "Person sentence is F**K!"
  }
}
// */




fn main() {
  let dog = Dog{};
  let cat = Cat{};
  dog.speak();
  cat.speak();

  let p1 = Person{};
  p1.speak();
}





