

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
    // where F: Fn(&str) -> ()     // todo: 匹配F是一个传入 &str 参数的函数
    where F: Fn(&str)      // todo: 返回单元值是默认值，可以省略这一部分
{
f("Hi!");
}

// todo: 显然，调用一个函数算是借用它, 编译错误, cannot borrow `visit` as mutable, as it is not declared as mutable
// pub fn comm3() {
//   let mut count = 0;
//   let visit = || {
//     count += 1;
//     println!("you are visitor #{}", count);
//   };
//
//   for _ in 1..6 {
//     visit();
//   }
// }

pub fn comm4() {
  let mut count = 0;
  let visit = || {
    count += 1;
    println!("you are visited #{}", count);
  };

  call_five_times(visit);     // todo: visit的整个生命周期交给call_five_times，这样就不会因为多次调用一个已经被借用的函数而报编译错误
}

fn call_five_times<F>(mut f: F)
  where
    F: FnMut(),     // todo: 需要持续调用，然后要修改的用FnMut
{
  for _ in 1..6 {
    f();
  }
}


pub fn comm5() {
  let name = String::from("Aclie");

  let welcome = || {
    let mut namex = name;
    namex += " and Bob";
    println!("Welcom {}", namex);
  };

  call_once(welcome);
}

fn call_once<F>(f: F)
where
    F: FnOnce(),   // todo: 当只调用一次，为了不借用报错，可以用FnMut
{
  f();
}























