use std::thread;

pub struct ThreadPool {
  threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
  /// 创建线程池。
    ///
    /// 线程池中线程的数量。
    ///
    /// # Panics
    ///
    /// `new` 函数在 size 为 0 时会 panic。
  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0);

    ThreadPool
  }

  pub fn execute<F> (&self, f: F)
      where
          F: FnOnce() + Send + 'static
  {

  }
}