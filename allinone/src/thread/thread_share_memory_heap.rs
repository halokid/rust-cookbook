
/*
堆
 */
use std::thread;
use std::sync::Arc;
// todo: 由于现代操作系统的设计，线程寄生于进程，可以共享进程的资源，如果要在各个线程中共享一个变量，那么除了上面的static，还有就是把变量保存在堆上了。当然Rust也不例外，遵从这一设计。只是我们知道Rust在安全性上肯定又会做一些考量，从而在语言设计和使用上稍有不同。 为了在堆上分配空间，Rust提供了std::boxed::Box，由于堆的特点，存活时间比较长，所以除了我们这个地方介绍的线程间共享外，还有其他的用处，此处不详细说明，若不甚了解，请学习或回顾堆、栈与Box章节的介绍。下面我们来看一下如何在多个线程间访问Box创建的变量：。

// todo: Box关键字会在堆上创建一个变量

// todo: 你可能会觉得很奇怪，上面怎么没有看到Box创建的变量啊，这明明就是Arc的使用呀？Box创建的变量要想在多个线程中安全使用，我们还需要实现很多功能才行，需要是Sync，而Arc正是利用Box来实现的一个通过引用计数来共享状态的包裹类。下面引用一段Arc::new的源码即可看出它是通过Box来实现的

pub fn comm() {
  let var: Arc<i32> = Arc::new(7);
  let share_var = var.clone();

  let new_thread = thread::spawn(move || {
    println!("share value in new thread: {}, address: {:p} ", share_var, &*share_var);
  });

  new_thread.join().unwrap();
  println!("share value in main thread: {}, address: {:p}", var, &*var);
}



