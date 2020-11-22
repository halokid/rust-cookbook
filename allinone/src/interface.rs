/*
接口
 */

// ------ 定义接口 -------
trait HasArea {
  fn area(&self) -> f64;
}

// ------- 实现接口 -----
struct Circle {
  x:  f64,
  y:  f64,
  radius:   f64,
}

impl HasArea for Circle {

  fn area(&self) -> f64 {
    std::f64::consts::PI * (self.radius *self.radius)
  }
}

pub fn comm1() {
  let c = Circle {
    x:  0.0f64,
    y:  0.0f64,
    radius:  1.0f64,
  };
  println!("圆的面积为 {}", c.area());
}



// -------- 定义接口2 ----------
// --- 定义 ---
trait Foo {
  fn foo(&self);
}

trait FooBar {
  fn foobar(&self);
}


// ---- 实现 ----
struct Baz;

impl Foo for Baz {
  fn foo(&self) {
    println!("struct impl trait foo");
  }
}

impl FooBar for Baz {

  fn foobar(&self) {
    println!("struct impl trait foobar");
  }
}


pub fn comm2() {
  let baz = Baz {};
  baz.foo();
  baz.foobar();
}

// ------- 自动化实现接口 ---------

// todo: Debug是一个自动化实现的接口, 装饰器的原理，系统自带的
#[derive(Debug)]
struct Foox {
  name:   String,
  age:    i32,
}

pub fn comm3() {
  // 自动化实现接口
  /*
  Rust会自动帮你实现接口，当然必须是某些特定的接口方法，就好像利用开发工具帮你实现一些接口方法一样。
能帮你实现的方法仅限于：Clone,Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd
这些方法一般是常见的方法，要自动实现上述方法，前提是使用derive属性。
   */
  // todo: 不实现 Debug 自动化方法， 就不能用println!输出
  println!("{:?}", Foox{ name: "halokid".to_string(), age: 17 });
}







