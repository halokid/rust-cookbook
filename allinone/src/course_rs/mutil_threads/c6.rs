use std::sync::Once;
use std::thread;

/*
只被调用一次的函数
有时，我们会需要某个函数在多线程环境下只被调用一次，例如初始化全局变量，无论是哪个线程先调用函数来初始化，都会保证全局变量只会被初始化一次，随后的其它线程调用就会忽略该函数。
代码运行的结果取决于哪个线程先调用 INIT.call_once （虽然代码具有先后顺序，但是线程的初始化顺序并无法被保证！因为线程初始化是异步的，且耗时较久），若 handle1 先，则输出 1，否则输出 2。
 */

static mut VAL: usize = 0;
static INIT: Once = Once::new();

pub fn comm() {
  let handle1 = thread::spawn(move || {
    INIT.call_once(|| {
      unsafe {
        VAL = 1;
      }
    });
  });

  let handle2 = thread::spawn(move || {
    INIT.call_once(|| {
      unsafe {
        VAL = 2;
      }
    });
  });

  handle1.join().unwrap();
  handle2.join().unwrap();

  println!("{}", unsafe { VAL });
}