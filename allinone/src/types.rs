/*
布尔值(bool)
字符(char)
有符号整型(i8, i16, i32, i64, i128)
无符号整型(u8, u16, u32, u64, u128)
指针大小的有符号/无符号整型(isize/usize，取决于计算机架构，32bit 的系统上，isize 等价于i32)
浮点数(f32, f64)
数组(arrays)，由相同类型元素构成，长度固定。
 */

pub fn comm() {
  let a = [1, 2, 3];
  let mut b = [1, 2, 3];

  let c: [i32; 3] = [1, 2, 3]; // [类型; 数组长度]

  // let d: ["my value"; 3]; //["my value", "my value", "my value"];

  let e: [i32; 0] = [];  // 空数组

  println!("{:?}", a);

  // 数组(arrays)的长度是可不变的，动态/可变长数组可以使用 Vec (非基本数据类型)。

  let a = (1, 1.5, true, 'a', "hello");
  println!("{}, {}", a.0, a.4);

  let b: (i32, f64) = (1, 1.5);

  let (c, d) = b;
  let (e, _, _, _, f) = a;  //e = 1, f = "Hello, world!", _ 作为占位符使用，表示忽略该位置的变量

  let g = (0,);     // 只包含一个元素的元组

  let h = (b, (2, 4), 5);     //((1, 1.5), (2, 4), 5)

  println!("{:?}", a);
}

pub fn slice() {
  let a: [i32; 4] = [1, 2, 3, 4];   // 定义一个数组

  let b: &[i32] = &a;     // 全部
  let c = &a[0..4];     // [0, 4]
  let d = &a[..];       // 全部

  let e = &a[1..3];     // [2, 3]
  let e = &a[1..];      // [2, 3, 4]
  let e = &a[..3];      // [1, 2, 3]
}

pub fn string() {
  // 在 Rust 中，str 是不可变的静态分配的一个未知长度的UTF-8字节序列。&str 是指向该字符串的切片。
  let a = "hello world";
  let b: &str = "halokid";

  // 字符串切片&str指向的字符串是静态分配的，在 Rust 中，有另一个堆分配的，可变长的字符串类型String(非基本数据类型)。通常由字符串切片&str通过 to_string() 或 String::from() 方法转换得到
  let s1 = "hello world".to_string();
  let s2 = String::from("hello world");
}




















