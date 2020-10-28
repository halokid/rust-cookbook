use std::sync::Arc;
use std::sync::mpsc;
use std::sync::Mutex;
use std::thread;

// todo: 会创建一个通道并充当发送端
pub struct ThreadPool {
  // 定义类， 通常属性会定义在这里
  workers: Vec<Worker>,
  sender: mpsc::Sender<Job>,
}

// todo: 存放用于向通道中发送的闭包
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
  // todo: 定义类的接口 或者 要执行的方法，
  /// 创建线程池。
    ///
    /// 线程池中线程的数量。
    ///
    /// # Panics
    ///
    /// `new` 函数在 size 为 0 时会 panic。
  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0);

    let (sender, receiver) = mpsc::channel();

    let receiver = Arc::new(Mutex::new(receiver));

    let mut workers = Vec::with_capacity(size);

    for id in 0..size {
//      workers.push(Worker::new(id, receiver));

      // todo: 线程安全智能指针
      workers.push(Worker::new(id, Arc::clone(&receiver)));
    }

    ThreadPool {
      workers,
      sender,
    }
  }

  pub fn execute<F>(&self, f: F)
    where
      F: FnOnce() + Send + 'static
  {
    let job = Box::new(f);

    self.sender.send(job).unwrap();
  }
}

// todo: 通道的接收端
struct Worker {
  // 定义类
  id: usize,
  thread: thread::JoinHandle<()>,
}

impl Worker {
  // 定义类接口
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
    let thread = thread::spawn(move || {
      loop {
        let job = receiver.lock().unwrap().recv().unwrap();

        println!("Worker {} got a job; executing.", id);

        job();
      }
    });

    Worker {
      id,
      thread,
    }
  }
}





