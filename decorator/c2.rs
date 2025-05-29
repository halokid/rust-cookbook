

macro_rules! log_decorator {
    ($func: expr) => {{
      println!("Before executing the function");
      let result = $func;
      println!("After executing the function");
      result
    }};
}

fn my_func(x: i32) -> i32 {
  println!("Inside my_function");
  x * 2
}

fn main() {
  let result = log_decorator!(my_func(3));
  println!("Result: {}", result);
}


