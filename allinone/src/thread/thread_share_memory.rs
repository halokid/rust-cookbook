
/*
异步通道channel
 */
use std::thread;
// todo: static如此，那const呢？ const会在编译时内联到代码中，所以不会存在某个固定的内存地址上，也不存在可以修改的情况，并不是内存共享的。
static VAR: i32 = 5;

static mut MUT_VAR: i32 = 6;

pub fn comm() {
  let new_thread = thread::spawn(move || {
    println!("static value in new thread: {}", VAR);

    // todo: 在不同线程要修改一个其他线程也可以读到的变量
    unsafe {
      println!("MUT_VAR in new_thread is: {}", MUT_VAR);
      MUT_VAR = MUT_VAR + 1;
    }
  });

  new_thread.join().unwrap();
  println!("static value in main thread: {}", VAR);

  unsafe {
    println!("MUT_VAR can be change, now in main thread is: {}", MUT_VAR);
  }
}



