/*
基于 Send 和 Sync 的线程安全
 */

use std::sync::Arc;
use std::thread;
use futures::TryFutureExt;
use tokio::sync::Mutex;

// todo: 总结
// todo: 1. 实现Send的类型可以在线程间安全的传递其所有权, 实现Sync的类型可以在线程间安全的共享(通过引用)
// todo: 2. 绝大部分类型都实现了Send和Sync，常见的未实现的有：裸指针、Cell、RefCell、Rc
// todo: 3. 可以为自定义类型实现Send和Sync，但是需要unsafe代码块
// todo: 4. 可以为部分 Rust 中的类型实现Send、Sync，但是需要使用newtype，例如文中的裸指针例子

pub fn comm() {
  /*
  // todo: 报错跟之前无二： `*mut u8` cannot be sent between threads safely, 但是有一个问题，我们无法为其直接实现Send特征，好在可以用newtype类型 :struct MyBox(*mut u8);。
  let p = 5 as *mut u8;
  let t = thread::spawn(move || {
    println!("{:?}", p);
  });
  t.join().unwrap();
   */
}

// -----------------------------------------------------------------------
#[derive(Debug)]
struct MyBox(*mut u8);

unsafe impl Send for MyBox {}

pub fn comm2() {
  let p = MyBox(5 as *mut u8);
  let t = thread::spawn(move || {
    println!("{:?}", p);
  });
  t.join().unwrap();
}
// todo: 此时，我们的指针已经可以欢快的在多线程间撒欢，以上代码很简单，但有一点需要注意：Send和Sync是unsafe特征，实现时需要用unsafe代码块包裹。


// -----------------------------------------------------------------------
/*
pub fn comm3() {
  // todo: 关于这种用法，在多线程章节也提到过，线程如果直接去借用其它线程的变量，会报错:closure may outlive the current function,, 原因在于编译器无法确定主线程main和子线程t谁的生命周期更长，特别是当两个线程都是子线程时，没有任何人知道哪个子线程会先结束，包括编译器！
  let v = 5;
  let t = thread::spawn(move || {
    println!("{:?}", &v);
  });
  t.join().unwrap();
}
 */

// todo: 因此我们得配合Arc去使用:
unsafe impl Sync for MyBox {}

/*
pub fn comm4() {
  let b = &MyBox(5 as *mut u8);
  // todo: Arc 可以在多线程中共享变量的所有权， Mutex可以在多线程中锁变量，避免数据竞争和污染
  let v = Arc::new(Mutex::new(b));
  // let vc = Arc::clone(&v);
  let t = thread::spawn(move || {
    let _v1 = v.lock().unwrap();
    // let _v1 = vc.lock().unwrap();
  });

  t.join().unwrap();
}
*/















