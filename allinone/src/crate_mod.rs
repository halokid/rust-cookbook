
/*
模块 让我们可以将一个 crate 中的代码进行分组，以提高可读性与重用性。即项是可以被外部代码使用的（public），还是作为一个内部实现的内容，不能被外部代码使用（private）。
 */
mod math {
  pub(crate) mod basic {
    pub(crate) fn plus(x: i32, y: i32) -> i32 { x + y }
    pub(crate) fn mul(x: i32, y: i32) -> i32 { x * y }
  }
}

pub fn comm() {
  println!("2 + 3 = {}", math::basic::plus(2, 3));
  println!("2 * 3 = {}", math::basic::mul(2, 3));
}





