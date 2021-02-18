/*
单例模式， 非线程安全， 在多线程环境下，会出现重复赋值情况
 */
#[derive(Debug)]
struct Singleton {
  v:    usize,
}

static mut SINGLETON_G:  Option<Singleton> = None;    // 初始化为None, SINGLETON_G就是单例

impl Singleton {
  fn new() -> &'static mut Singleton {
    unsafe {      // Rust中使用可变静态变量都要声明unsafe
      match SINGLETON_G {
        Some(ref mut obj) => obj,     // 假如是一个引用的obj
        None => {
          SINGLETON_G = Some(Singleton{ v: 100 });
          Singleton::new()
        }
      }
    }
  }
}

fn main() {
  let s1 = Singleton::new();
  let s2 = Singleton::new();
  println!("s1: {:?}", s1);
  println!("s2: {:?}", s2);

  s1.v = 999;
  println!("s1: {:?}", s1);
  println!("s2: {:?}", s2);
}



