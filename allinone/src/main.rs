pub mod types;
pub mod operation;
pub mod control;
pub mod impl_trait;
pub mod generics;
pub mod vec;
pub mod string;
pub mod hashmap;
pub mod errors_handle;
pub mod crate_mod;
pub mod inherit;
pub mod interface;
pub mod config;
pub mod read_config;
pub mod config3;
pub mod person;
pub mod clone;
pub mod loopx;
pub mod foobar;
pub mod mysql;
pub mod json_string;
pub mod async_block;
pub mod async_no_block;
pub mod async_no_block1;
pub mod closure_futures_async_await;

use log::debug;
use simplelog::{ConfigBuilder, LevelFilter, SimpleLogger};

// extern crate local_ipaddress;  // 引入外部crate 方式2
use local_ipaddress;
use futures::{Future, FutureExt};        // 引入外部的crate 方式1
use futures;
use std::pin::Pin;
use std::error::Error;
use std::convert::Infallible;
use std::time::Duration;
use tokio::time::delay_for;

// todo: Box的返回是表示该函数返回的数据 夺取了控制权
// todo: dyn 是表示 trait 的多态
// todo: 不返回Pin的话会报错, trait `std::marker::Unpin` is not implemented for `dyn core::future::future::Future<Output = i32>
fn returns_dyn_future_i32() -> Pin<Box<dyn Future<Output = i32>>> {
  if rand::random() {
    Box::pin((futures::future::ready(42)))
  } else {
    Box::pin(futures::future::lazy(|_| 1337))
  }
}

// todo: Infallible 是返回特定的类型， 并且有这个类型的内存声明和长度的
// todo: Infallible 跟返回一个类型的实行用 Box 包住的作用差不多
fn return_future_result() -> impl Future<Output = Result<i32, impl Error>> {
  futures::future::ok::<i32, Infallible>(42)
}

// todo: 线程延时
fn returns_delayed_future() -> impl Future<Output = i32> {
  delay_for(Duration::from_secs(3))
    .then(|_| futures::future::ready(42))
}



#[allow(dead_code)]
fn main() {
  // init log, 初始化log模块
  let config = ConfigBuilder::new()
    .set_target_level(LevelFilter::Trace)
    .build();
  let _ = SimpleLogger::init(LevelFilter::Debug, config);


  // json_string::comm();
  // async_block::comm();
  // async_no_block::comm();
  // async_no_block1::comm();

  // todo: 引用文件夹里面的函数
  // closure_futures_async_await::p1::comm();
  // closure_futures_async_await::p2::comm();
  // closure_futures_async_await::p3::comm();
  // closure_futures_async_await::p3::comm2();
  // closure_futures_async_await::p5::comm1();
  // futures::future::ready(42);     // todo: 这样是不会执行的, 加上.await也不行

  let mut rt = tokio::runtime::Runtime::new().unwrap();
  {
    let result = rt.block_on(futures::future::ready("x rt.block_on()"));
    debug!("{}", result);
  }

  rt.block_on(returns_dyn_future_i32());

  /*
  std::process::exit(0);

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
  // =========== dny traits =================
  impl_trait::comm();

  // ================= 泛型 ======================
  generics::comm();

  // ================= Vec集合 ==================
  vec::comm();

  // ================ String 集合 ================
  string::comm();

  person::comm();

  clone::comm();

  loopx::comm();

  foobar::comm();



  // mysql::comm();

  /*
  // =============== Hashmap 集合 ===============
  hashmap::comm();

  // =============== 错误处理 ==================
  errors_handle::comm();

  // ========== 包、crate和模块 ================
  crate_mod::comm();

  // ========== 调用外部crate =================
  let ipaddr = local_ipaddress::get().unwrap();
  println!("ipaddr: {}", ipaddr);

  // ============ 继承范例1 ================
  inherit::comm();

  // =========== 接口范例1 ===============
  interface::comm1();
  interface::comm2();
  interface::comm3();

  // ========== 读取配置 =================
  config::comm();     // 1
  read_config::comm();      // 2
  config3::comm();

   */
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


// =============== 单元测试 ===============
fn plus(x: i32, y: i32) -> i32 {
  x + y
}

#[test]
fn it_works() {
  assert_eq!(4, plus(2, 2));
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_worksx() {
    assert_eq!(4, plus(2, 2));
  }

   */
}





