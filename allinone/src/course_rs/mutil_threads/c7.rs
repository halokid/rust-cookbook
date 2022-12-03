/*
线程间通信：标准库提供了通道std::sync::mpsc，其中mpsc是multiple producer, single consumer的缩写，代表了该通道支持多个发送者，但是只支持唯一的接收者。 当然，支持多个发送者也意味着支持单个发送者，我们先来看看单发送者、单接收者的简单例子:
 */

use std::sync::mpsc;
use std::thread;

pub fn comm() {
  // 创建一个消息通道, 返回一个元组：(发送者，接收者)
  let (tx, rx) = mpsc::channel();

  // 创建线程，并发送消息
  thread::spawn(move || {
    // 发送一个数字1, send方法返回Result<T,E>，通过unwrap进行快速错误处理
    tx.send(1).unwrap();

    // 下面代码将报错，因为编译器自动推导出通道传递的值是i32类型，那么Option<i32>类型将产生不匹配错误
    // tx.send(Some(1)).unwrap()
  });

  // 在主线程中接收子线程发送的消息并输出
  println!("receive {}", rx.recv().unwrap());

  // todo: 不阻塞的 try_recv 方法, 除了上述recv方法，还可以使用try_recv尝试接收一次消息，该方法并不会阻塞线程，当通道中没有消息时，它会立刻返回一个错误
  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
    tx.send(1).unwrap();
  });

  println!("receive {:?}", rx.try_recv());
  println!("receive {:?}", rx.try_recv());
  println!("receive {:?}", rx.try_recv());
}

