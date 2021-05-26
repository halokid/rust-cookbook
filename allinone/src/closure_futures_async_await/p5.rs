use log::debug;

pub fn comm1() {
  // todo: 建立一个常规的异步runtime
  // let mut rt = tokio::runtime::Runtime::new().unwrap();

  // todo: 建立一个可trace的异步runtime
  let mut rt = tokio::runtime::Builder::new()
        .threaded_scheduler()
        .core_threads(4)
        .on_thread_start(|| debug!("on_thread_start()"))
        .build()
        .unwrap();

  rt.enter(|| {
    debug!("in rt.enter()");
    tokio::spawn(futures::future::lazy(|_| debug!("in tokio::spawn()")));
  });

  rt.spawn(futures::future::lazy(|_| debug!("in rt.spawn()")));

  rt.block_on(futures::future::lazy(|_| debug!("in rt.block_on()")));

  debug!("只有rt.enter() 和 rt.block_on() 是跑在1线程(main线程)")
}