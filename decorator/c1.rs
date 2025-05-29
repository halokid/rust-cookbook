
fn decorator<F>(func: F) -> impl Fn(i32) -> i32
where
  F: Fn(i32) -> i32,
{
  move |x| {
    println!("Before executing the func");
    let result = func(x);
    println!("After executing the func");
    result
  }
}

fn my_func(x: i32) -> i32 {
  println!("Inside my_func");
  x * 2
}

fn main() {
  let decoratro_func = decorator(my_func);

  let result = decoratro_func(3);
  println!("Result: {}", result);
}


