pub mod types;
pub mod operation;
pub mod control;
pub mod impl_trait;
pub mod generics;
pub mod vec;
pub mod string;

fn main() {
  println!("Hello, world!");
  println!("{}, {}!", "hello", "world");
  println!("{0}, {1}", "hello", "world");
  println!("{greeting}, {name}", greeting="hello", name="world");

  let y = String::from("hello, ") + "world";
  println!("{}", y);

  /* --------------- 变量 -------------- */
  //  不可变变量
  let c;
  let a = true;
  let b: bool = true;
  let (x, y) = (1, 2);
  c = 12345;

  // 可变变量
  let mut z = 5;
  z = 6;

  // 静态变量（不可变）
  static N: i32 = 5;

  // 静态变量（可变）
  static mut NX: i32 = 5;

  // 常量
  const NXX: i32 = 5;

  let (add_num, result) = plus_onex(10);
  println!("{} + 1 = {}", add_num, result);

  let b = plus_one;     // 函数指针赋值给变量
  let c = b(5);    // 调用变量等于调用函数

  // ================= 基本数据类型 ===================
  types::comm();

  // ================= 操作符 ====================
  operation::comm();
  operation::dereference();

  // ================= 控制符 ===================
  control::comm();

  // ================= 实现方法和接口 ================
  impl_trait::comm();

  // ================= 泛型 ======================
  generics::comm();

  // ================= Vec集合 ==================
  vec::comm();

}

/// 外部注释
mod test {
  // 行注释
  /* 块注释 */
}

mod testx {
  //! 包/模块级别的注释

  // ...
}


/* ============= 函数部分 =============== */
fn print_sum(a: i8, b: i8) {
  println!("sum is: {}", a + b);
}

fn plus_one(a: i32) -> i32 {
  a + 1
  // 等价于 return a + 1，可省略为 a + 1
}

fn plus_onex(a: i32) -> (i32, i32) {
  (a, &a + 1)
}










