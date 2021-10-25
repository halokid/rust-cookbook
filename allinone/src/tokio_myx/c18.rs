// todo: 异步场景9： 1. 普通fn 调用 async fn， 并且获取async  fn的返回, 这种情况的话普通fn一定要block_on（阻塞）获取到 async fn的返回的, 因为不阻塞就获取不了async函数的实时返回， 如果要异步返回的话，那就是另外的场景了    2. 异步函数转换为同步函数

use futures::executor;
use async_std;
use futures::{future, Future};


pub fn comm() {
  // todo: futures 方式
  let v = executor::block_on(sample());
  println!("futures方式noasync fn获取async fn 的返回: {}", v);

  // todo: tokio 方式, 实际上也是future的封装和优化
  // todo: 这种方式阿不能用在本身出于协程调度的 普通 fn 里面的， 不然会产生这个错误 thread 'tokio-runtime-worker' panicked at 'Cannot start a runtime from within a runtime. This happens because a function (like `block_on`) attempted to block the current thread while the thread is being used to drive asynchronous tasks.'
  let vx = tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .unwrap()
    .block_on(sample());
  println!("tokio 1 方式noasync fn获取async fn 的返回: {}", vx);

  // 上面的简化写法
  let vxx = tokio::runtime::Runtime::new().unwrap().block_on(sample());
  println!("tokio 2 方式noasync fn获取async fn 的返回: {}", vxx);


  // let s = example().await;
  let vxxx = executor::block_on(example());
  println!("async函数的future写法 ------- {}", vxxx);
}

// todo: 使用 async_std::main 属性main函数将其从异步函数转换为同步函数
// #[async_std::main]
// async fn foo() {
//   let v = sample().await;
//   println!("async 换位 sync 函数获取返回: {}", v);
// }

async fn sample() -> String {
    return "hello".to_string();
}

// todo: ----- 关于async的本质转化, 这里就相当于 async函数了 ------
// todo: 下面相当于 async fn example() -> String { }
fn example() -> impl Future<Output = String> {
  async {
    let ret = sample().await;
    ret
  }
}



