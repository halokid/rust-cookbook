
// receive closures, 执行一个闭包
fn receives_closure<F>(closure: F)
where
    F:  Fn(i32) -> i32,
{
  let result = closure(1);
  println!("closure(1) => {}", result);
}

// 返回一个闭包
fn returns_closure() -> impl Fn(i32) -> i32 {
  |x| x + 4
}

// todo: F是一个泛型
fn curr<F>(f: F, x: i32) -> impl Fn(i32) -> i32
where
    F: Fn(i32, i32) -> i32,
{
  move |y| f(x, y)
}




pub fn comm() {
  // let add = |x| x + 2;

  /*
  let y = 2;
  let add = |x| x + y;

  let z = 3;
  let addx = |x| x + z;

  receives_closure(addx);
   */

  // todo: 要定义两个不同的作用域， 不然 x 变量会冲突报错
  {
    let y = 2;
    receives_closure(|x| x + y);
  }

  {
    let y = 3;
    receives_closure(|x| x + y);
  }

  // let closure = returns_closure();
  // println!("closure(1) => {}", closure(1));

  // todo: 调用一个fn的泛型， 而且这个fn是一个闭包（匿名函数）
  let add = |x, y| x + y;
  let closure = curr(add, 5);
  println!("closure(1) => {}", closure(1));
}







