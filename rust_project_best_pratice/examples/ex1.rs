use porust::*;      // todo: 全局引用porust， 这是引用一个create
use porust::bar::bar_say_foo;     // todo: 局部引用某个文件、函数等

/*
todo: 执行example的范例， 在根目录运行 cargo run --example ex1 可以执行该example, ex1为文件名
*/

fn main() {
  println!("ex1 example");
  server_ex1();

  server_ex2();

  server_ex3();

  server_ex4();
}

fn server_ex1() {
  // todo: 假如lib没有额外配置， test() 函数必须放在默认的lib.rs才能装载出来
  porust::test();
}


fn server_ex2() {
  porust::foo::say_foo();
}

fn server_ex3() {
  println!("{}", porust::GREETING);
}

fn  server_ex4() {
  porust::bar::bar_say_foo();
  bar_say_foo();
}