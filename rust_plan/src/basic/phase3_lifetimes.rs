/**
为什么需要生命周期

编译器需要知道引用的作用域是否有效

*/

/**
函数中生命周期的意义

'a 表示：返回值引用的生命周期至少与参数中较短的引用一样长
*/

/**
练习：

写一个带有生命周期的函数，它接受两个字符串引用，返回较长者
*/
pub fn longest<'a> (x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}


pub fn take_lifetimes<'a, 'b> (x: &'a str, y: &'b str) -> &'b str {
  return y;
}


