

pub fn comm() {
  let msg = "Hi!";

  // fn say_hi() {
  //   println!("say hi {}", msg);
  // }

  let say_hi = || {
    println!("{}", msg);
  };
  say_hi();
  say_hi();

  let say_hi2 = |msg| {
    println!("{}", msg);
  };
  say_hi2(msg);
  say_hi2(msg);
}

pub fn comm2() {
  let say_msg = |msg: &str| println!("{}", msg);
  // call_with_hi(say_msg);
  // call_with_hi(say_msg);

  call_with_hi2(say_msg);
}

// todo: 报错！ 编译器不知道 f 是一个函数。 现在该终于可以介绍编译的魔力了： Fn trait！
// fn call_with_hi<F> (f: F) {      // todo: <F> 表示返回一个函数
//   f("Hi!");
// }

fn call_with_hi2<F> (f: F)
    where F: Fn(&str) -> ()     // todo: 匹配F是一个传入 &str 参数的函数
{
  f("Hi!");
}











