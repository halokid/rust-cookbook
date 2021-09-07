
/*
channel 发送消息的类型
 */
/*
use std::sync::mpsc;
use std::thread;
use std::fmt;
use std::rc::Rc;
use std::fmt::Formatter;

pub struct Student {
  id:     u32,
}

impl fmt::Display for Student {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "student {}", self.id)
  }
}

pub fn comm() {
  // 创建一个通道
  let (tx, rx): (mpsc::Sender<Rc<Student>>, mpsc::Receiver<Rc<Student>>) =
    mpsc::channel();

  // 创建线程用于发送消息
  thread::spawn(move || {
    // 发送一个消息， 此处是数字id
    tx.send(Rc::new(
      Student{ id: 1 }
    )).unwrap();
  });

  // 在主线程接收子线程发送的消息并输出
  println!("receive {}", rx.recv().unwrap());
}

 */



