
// 实现方法(impl)
struct Rectangle {
  width:      u32,
  height:     u32,
}

impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }
}

impl Rectangle {
  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}

impl Rectangle {
  fn square(size: u32) -> Rectangle {
    Rectangle{ width: size, height: size }
  }
}



// ============ traits ================
// 实现接口(traits)
trait Summary {
  fn summarize(&self) -> String;
}

impl Summary for Rectangle {
  fn summarize(&self) -> String {
    format!("width={}, height={}", self.width, self.height)
  }
}

// 接口也支持继承
trait Person {
  fn full_name(&self) -> String;
}

// Employee 继承了 Person 的特性(trait)
trait Employee: Person {
  fn job_title(&self) -> String;
}

trait Expat {
  fn salary(&self) -> f32;
}

trait ExpatEmployee: Employee + Expat {  // 多继承，同时继承 Employee 和 Expat
  fn additional_tax(&self) -> f64;
}



// ============== returning traits with dyn =======================
struct Sheep{}
struct Cow{}

trait Animal {
  fn noise(&self) -> &'static str;
}

// todo: 为 Sheep 实现 Animal特性
impl Animal for Sheep {
  fn noise(&self) -> &'static str {
    "baaaaaaaaaaa!"
  }
}

// todo: 为 Cow 实现 Animal特性
impl Animal for Cow {
  fn noise(&self) -> &'static str {
    "mooooooooo!"
  }
}

// todo: 返回某个struct可以执行 Animal 的特性， 但是我们不知道在编译的时候究竟具体是返回哪个
// todo: dyn 是动态的意思， 意思就是这个返回的值不确定是哪一个
fn random_animal(random_number: f64) -> Box<dyn Animal> {
  if random_number < 0.5 {
    Box::new(Sheep{})
  } else {
    Box::new(Cow{})
  }
}


pub fn comm() {
  let rect1 = Rectangle{ width: 30,  height: 50 };

  println!("面积为: {}", rect1.area());

  let rect2 = Rectangle::square(9);
  println!("正方形面积为: {}", rect2.area());
  let summar = rect2.summarize();
  println!("summar: {}", summar);

  let random_number = 0.24;
  let animal = random_animal(random_number);
  println!("dyn train return Animal: {}", animal.noise());
}





