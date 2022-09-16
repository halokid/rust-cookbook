
// todo: if code builds has warnning, it will build fail!!!
// #![deny(warnings)]

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
pub mod encrypt;
pub mod encrypt2;
pub mod aes;
pub mod aes2;
pub mod aes3;
pub mod aes4;
pub mod hmac;
pub mod hmac2;
pub mod base64;
pub mod mycrypto_encode;
pub mod microsoft_guide;
pub mod signal;
pub mod timex;
pub mod serial_number;
pub mod primitives;
pub mod cmdargs;
pub mod structx;
pub mod futures_sample;
pub mod concurrence_parallel_goAndRust;
pub mod tokio_my;
pub mod tokio_myx;
pub mod channel_compare_tokio_std;
pub mod rust_design_patterns;
pub mod oop;
pub mod data_structures_algorithms;
pub mod feature;
pub mod fealess_concurrency;
pub mod cooking_with_rust;
pub mod course_rs;

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
// use tokio::time::delay_for;

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
// fn returns_delayed_future() -> impl Future<Output = i32> {
  // delay_for(Duration::from_secs(3))
  //   .then(|_| futures::future::ready(42))
// }

pub trait SerialNumber: Sized + Copy + Eq {
  fn sn_add(self, n: Self) -> Option<Self>;
  fn sn_lt(self, n: Self) -> bool;
}

/*
 The module contains implementations of SerialNumber for u8, u16, u32, u64;
 in those, SERIAL_BITS is equal to each type's bit count.
 */

// todo: arbitrary SERIAL_BITS up to 64

// todo: arbitrary SERIAL_BITS above 64

#[allow(dead_code)]
fn main() {
  // init log, 初始化log模块
  let config = ConfigBuilder::new()
    .set_target_level(LevelFilter::Trace)
    .build();
  let _ = SimpleLogger::init(LevelFilter::Debug, config);

  // encrypt::comm1();
  // encrypt2::comm();
  // aes::comm();
  // aes2::comm();
  // aes4::aes_cbc_mode();
  // hmac::comm();
  // base64::comm();
  // hmac2::comm();
  // mycrypto_encode::comm();
  // microsoft_guide::car::comm();
  // microsoft_guide::loop_return::comm();
  // microsoft_guide::error_handle::comm();
  // microsoft_guide::error_handle::comm3();
  // microsoft_guide::error_handle_result::comm();
  // microsoft_guide::error_handle_result::comm2();
  // microsoft_guide::ownship::comm();
  // microsoft_guide::ownship_borrow::comm();
  // microsoft_guide::generic::comm();
  // microsoft_guide::trait_case::comm2();
  // microsoft_guide::trait_case2::comm();
  // microsoft_guide::iterator::comm();
  // microsoft_guide::genric2::comm();
  // microsoft_guide::iterator2::comm();
  // microsoft_guide::package_crate_mod_guide::comm3();
  // signal::comm();
  // timex::comm();
  // serial_number::comm();
  // primitives::comm();
  // cmdargs::comm();
  // structx::c1::comm();
  // futures_sample::c1::comm();
  // thread::c1::comm();
  // thread::c2::comm();
  // thread::c3::comm();
  // thread::thread_comiunica_channel::comm();
  // thread::thread_comiunica_channel2::comm();
  // thread::thread_async_channel::comm();
  // thread::thread_sync_channel::comm();
  // thread::thread_share_memory::comm();
  // thread::thread_notify::comm();
  // concurrence_parallel_goAndRust::c1::comm();
  // concurrence_parallel_goAndRust::c2::comm();
  // tokio_my::c1::comm();
  // tokio_my::c2::comm();
  // std::process::exit(0);
  // tokio_myx::c1::comm();
  // tokio_myx::c2::comm();
  // tokio_myx::c3::comm();
  // tokio_myx::c4::comm();
  // tokio_myx::c5::comm();
  // tokio_myx::c6::comm();
  // tokio_myx::c7::comm();
  // tokio_myx::c8::comm();
  // tokio_myx::c9::comm();
  // tokio_myx::c10::comm();
  // tokio_myx::c11::comm();
  // tokio_myx::c12::comm();
  // tokio_myx::c13::comm();
  // tokio_myx::c14::comm();
  // tokio_myx::c15::comm();
  // tokio_myx::c16::comm();
  // tokio_myx::c18::comm();
  // tokio_myx::c19::comm();
  // channel_compare_tokio_std::std_channel1::comm();
  // channel_compare_tokio_std::tokio_channel1::comm();
  // rust_design_patterns::idioms::use_borrowed_type_for_fn::comm();
  // rust_design_patterns::idioms::default_trait::comm();
  // rust_design_patterns::idioms::finalisation_destructor::comm();
  // rust_design_patterns::design_patterns::behavioural::command1::comm();
  // rust_design_patterns::design_patterns::behavioural::command2::comm();
  // rust_design_patterns::design_patterns::behavioural::interpreter1::comm();
  // rust_design_patterns::design_patterns::behavioural::strategy::comm();
  // rust_design_patterns::design_patterns::behavioural::stradegy_closure::comm();
  // rust_design_patterns::design_patterns::creational::builder::comm();
  // feature::rc_arc::comm();
  // rust_design_patterns::design_patterns::anti_patterns::deref_polymorphism::comm();
  // feature::arc_sample::comm();
  // data_structures_algorithms::chapter01::comm::comm();
  // fealess_concurrency::listing_16_12::comm();
  // fealess_concurrency::listing_16_13::comm();
  // oop::single_struct_circle::comm();
  // oop::c1::comm1();
  // let s = tokio_myx::c19::comm2().await;
  // println!("s ---- {}", s);
  // std::thread::sleep(Duration::from_secs(10));
  // std::process::exit(0);

  // json_string::comm();
  // cooking_with_rust::concurrency::spawn_a_shortlived_thread::comm();
  // async_block::comm();
  // async_no_block::comm();
  // async_no_block1::comm();
  // course_rs::variable_type::c1::comm();
  // course_rs::variable_type::c2::comm();
  // course_rs::ownership_borrow::c2::comm();
  // course_rs::complex_type::string_allinone::comm();
  // course_rs::complex_type::structx::comm();
  // course_rs::complex_type::enumeration::comm();
  // course_rs::complex_type::optionx::comm();
  // course_rs::flow_control::iflese::comm();
  // course_rs::flow_control::matchx::comm();
  // course_rs::generic_trait::c1::comm();
  // course_rs::generic_trait::c2::comm();
  // course_rs::generic_trait::c4::comm();
  // course_rs::generic_trait::c6::comm();
  // course_rs::generic_trait::c7::comm();
  // course_rs::collections::c1::comm();
  // course_rs::collections::c2::comm();
  // course_rs::type_conversion::c1::comm();
  // course_rs::error_handle::c1::comm();
  // course_rs::error_handle::c2::comm();
  // course_rs::comm::comm();
  // course_rs::life_cycle::c1::comm();
  // course_rs::life_cycle::c4::comm();
  // course_rs::closure_iterate::c1::comm();
  course_rs::deep_type::c1::comm();

  // todo: 引用文件夹里面的函数
  // closure_futures_async_await::p1::comm();
  // closure_futures_async_await::p2::comm();
  // closure_futures_async_await::p3::comm();
  // closure_futures_async_await::p3::comm2();
  // closure_futures_async_await::p5::comm1();
  // futures::future::ready(42);     // todo: 这样是不会执行的, 加上.await也不行

  /*
  let mut rt = tokio::runtime::Runtime::new().unwrap();
  {
    let result = rt.block_on(futures::future::ready("x rt.block_on()"));
    debug!("{}", result);
  }

  rt.block_on(returns_dyn_future_i32());
   */

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





