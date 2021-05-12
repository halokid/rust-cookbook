
// rust的闭包

fn recrives_closure<F>(closure: F)    // todo: 泛型
where
    F:  Fn(i32, i32) -> i32,      // 匹配到closure的类型、参数等
{
  let result = closure(1, 2);
  println!("closure(1, 2) => {}", result);
}

pub fn comm() {
  let add = |x, y| x + y;
  recrives_closure(add);
}

