pub mod foo;      // todo: 让编译器装载foo mod, 这是在lib文件之外定义lib逻辑代码的方式
pub mod bar;

pub const GREETING: &'static str = "Hallo, Rust library here!";

pub fn test() {
  println!("server mod test...");
}

