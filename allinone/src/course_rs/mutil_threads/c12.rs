// todo: 互斥锁, 既然是共享内存，那并发原语自然是重中之重，先来一起看看皇冠上的明珠: 互斥锁Mutex(mutual exclusion 的缩写)。Mutex让多个线程并发的访问同一个值变成了排队访问：同一时间，只允许一个线程A访问该值，其它线程需要等待A访问完成后才能继续。

use std::sync::Mutex;

pub fn comm() {
  // todo: 单线程中使用Mutex
  // 使用`Mutex`结构体的关联函数创建新的互斥锁实例
  let m = Mutex::new(5);

  {
    // 获取锁，然后deref为`m`的引用
    // lock返回的是Result
    let mut num = m.lock().unwrap();
    *num = 6;
    // 锁自动被drop
  }
  /*
  这里你可能奇怪，m.lock明明返回一个锁，怎么就变成我们的num数值了？聪明的读者可能会想到智能指针，没错，因为Mutex<T>是一个智能指针，准确的说是m.lock()返回一个智能指针MutexGuard<T>:
  它实现了Deref特征，会被自动解引用后获得一个引用类型，该引用指向Mutex内部的数据
它还实现了Drop特征，在超出作用域后，自动释放锁，以便其它线程能继续获取锁
   */

  println!("m = {:?}", m);
}

pub fn comm2() {
  let m = Mutex::new(5);

  let mut num = m.lock().unwrap();
  *num = 6;
  // 锁还没有被 drop 就尝试申请下一个锁，导致主线程阻塞
  // drop(num); // 手动 drop num ，可以让 num1 申请到下个锁
  let mut num1 = m.lock().unwrap();
  *num1 = 7;
  // drop(num1); // 手动 drop num1 ，观察打印结果的不同

  println!("m = {:?}", m);
}


