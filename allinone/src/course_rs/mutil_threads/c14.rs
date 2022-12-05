use std::sync::{Mutex, MutexGuard, RwLock};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

pub fn comm() {
  // todo: 单线程死锁
  // let data = Mutex::new(0);
  // let d1 = data.lock();
  // let d2 = data.lock();   // todo: 死锁
}

// todo: 多线程死锁
use lazy_static::lazy_static;

lazy_static! {
  static ref MUTEX1: Mutex<i64> = Mutex::new(0);
  static ref MUTEX2: Mutex<i64> = Mutex::new(0);
}

pub fn comm2() {
  // 存放子线程的句柄
  let mut children = vec![];
  for i_thread in 0..2 {
    children.push(thread::spawn(move || {
      for _ in 0..1 {
        // 线程1
        if i_thread % 2 == 0 {
          // 锁住MUTEX1
          let guard: MutexGuard<i64> = MUTEX1.lock().unwrap();

          println!("线程 {} 锁住了MUTEX1，接着准备去锁MUTEX2 !", i_thread);

          // 当前线程睡眠一小会儿，等待线程2锁住MUTEX2
          sleep(Duration::from_millis(10));

          // 去锁MUTEX2
          // let guard = MUTEX2.lock().unwrap();
          // todo: 与lock方法不同，try_lock会尝试去获取一次锁，如果无法获取会返回一个错误，因此不会发生阻塞:
          let guard = MUTEX2.try_lock();
          println!("线程1获取MUTEX2锁的结果: {:?}", guard);
          // 线程2
        } else {
          // 锁住MUTEX2
          let _guard = MUTEX2.lock().unwrap();

          println!("线程 {} 锁住了MUTEX2, 准备去锁MUTEX1", i_thread);

          // let _guard = MUTEX1.lock().unwrap();
          sleep(Duration::from_millis(10));
          let guard = MUTEX1.try_lock();
          println!("线程2获取MUTEX1锁的结果: {:?}", guard);
        }
      }
    }));
  }

  // 等子线程完成
  for child in children {
    let _ = child.join();
  }

  println!("死锁没有发生");
}
/*
在上面的描述中，我们用了"可能"二字，原因在于死锁在这段代码中不是必然发生的，总有一次运行你能看到最后一行打印输出。这是由于子线程的初始化顺序和执行速度并不确定，我们无法确定哪个线程中的锁先被执行，因此也无法确定两个线程对锁的具体使用顺序。

但是，可以简单的说明下死锁发生的必然条件：线程 1 锁住了MUTEX1并且线程2锁住了MUTEX2，然后线程 1 试图去访问MUTEX2，同时线程2试图去访问MUTEX1，就会死锁。 因为线程 2 需要等待线程 1 释放MUTEX1后，才会释放MUTEX2，而与此同时，线程 1 需要等待线程 2 释放MUTEX2后才能释放MUTEX1，这种情况造成了两个线程都无法释放对方需要的锁，最终死锁。

那么为何某些时候，死锁不会发生？原因很简单，线程 2 在线程 1 锁MUTEX1之前，就已经全部执行完了，随之线程 2 的MUTEX2和MUTEX1被全部释放，线程 1 对锁的获取将不再有竞争者。 同理，线程 1 若全部被执行完，那线程 2 也不会被锁，因此我们在线程 1 中间加一个睡眠，增加死锁发生的概率。如果你在线程 2 中同样的位置也增加一个睡眠，那死锁将必然发生!
 */

pub fn comm3() {
  let lock = RwLock::new(5);

  // 同一时间允许多个读
  {
    let r1 = lock.read().unwrap();
    let r2 = lock.read().unwrap();
    assert_eq!(*r1, 5);
    assert_eq!(*r2, 5);
  } // 读锁在此处被drop

  // 同一时间只允许一个写
  {
    let mut w = lock.write().unwrap();
    *w += 1;
    assert_eq!(*w, 6);

    // 以下代码会panic，因为读和写不允许同时存在
    // 写锁w直到该语句块结束才被释放，因此下面的读锁依然处于`w`的作用域中
    // let r1 = lock.read();
    // println!("{:?}",r1);
  }// 写锁在此处被drop
}

/*
Mutex 还是 RwLock
首先简单性上Mutex完胜，因为使用RwLock你得操心几个问题：

读和写不能同时发生，如果使用try_xxx解决，就必须做大量的错误处理和失败重试机制
当读多写少时，写操作可能会因为一直无法获得锁导致连续多次失败(writer starvation)
RwLock 其实是操作系统提供的，实现原理要比Mutex复杂的多，因此单就锁的性能而言，比不上原生实现的Mutex
再来简单总结下两者的使用场景：

追求高并发读取时，使用RwLock，因为Mutex一次只允许一个线程去读取
如果要保证写操作的成功性，使用Mutex
不知道哪个合适，统一使用Mutex
需要注意的是，RwLock虽然看上去貌似提供了高并发读取的能力，但这个不能说明它的性能比Mutex高，事实上Mutex性能要好不少，后者唯一的问题也仅仅在于不能并发读取。

一个常见的、错误的使用RwLock的场景就是使用HashMap进行简单读写，因为HashMap的读和写都非常快，RwLock的复杂实现和相对低的性能反而会导致整体性能的降低，因此一般来说更适合使用Mutex。

总之，如果你要使用RwLock要确保满足以下两个条件：并发读，且需要对读到的资源进行"长时间"的操作，HashMap也许满足了并发读的需求，但是往往并不能满足后者："长时间"的操作。

benchmark 永远是你在迷茫时最好的朋友！
 */




