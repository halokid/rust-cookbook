
/**
什么是所有权？

每个值在任一时刻只能有一个“主人”

当变量离开作用域，值会被释放（drop）
*/


/**
练习：

编写函数，将 String 所有权传入函数体中，再返回。
*/
pub fn c1(s: String) -> String {
  // let sx = format!("{} in plus", s);
  // return sx;

  return s;
}

