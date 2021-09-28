use std::thread::sleep;
use std::thread;
use tokio::time::Duration;
use std::time;
use tokio::runtime::Builder;

// todo: 异步场景2： 在一个非async的函数里调用async的函数
pub fn comm() {
  // noasync_call_async();   // todo: 这样可以编译过，但是没用， 因为这里不会执行

  // noasync_call_async().await;   // todo: await才是async fn执行的标识, 编译错误 ^^^^ only allowed inside `async` functions and blocks

  // async {
  //   noasync_call_async();    // todo: 这样可以编译过，但是没用， 因为这里不会执行
  // };

  // tokio::task::spawn(async {
  //   noasync_call_async();    // todo: 编译错误, thread 'main' panicked at 'must be called from the context of a Tokio 0.3.x runtime configured with either `basic_scheduler` or `threaded_scheduler`', src\tokio_myx\c11.rs:17:3,  这个原因是因为  tokio::spawn 的逻辑必须运行在 #[tokio::main] 装饰的函数里面, 也可以不这样， 但是你必须自己建一个 tokio 的runtime let mut rt = tokio::runtime::Runtime::new().unwrap();, 然后用这个runtime去调度线程
  // });

  let mut async_ret = "在async spawn外面的async_ret".to_string();
  // todo: 自己建一个 runtime 的方式
  let runtime = Builder::new_multi_thread().
    max_threads(3).   // todo: 大于 0 就可以并发
    enable_all().
    build().
    expect("create tokio runtime failed");

  runtime.spawn(async move {
    noasync_call_async().await;   // todo: 在一个常规的fn里调用async fn 的正确姿势1, 自己建立一个tokio的runtime， 然后调度async fn, 跟fn本身就是async装饰的道理本质上是一样的

    noasync_call_nromal();

    // async_ret = noasync_call_async_res().await;
    let async_ret_x = noasync_call_async_res().await;
    println!("直接在普通的fn 里面的async fn返回: {}", async_ret_x);
    return;
    // async_ret = async_ret_x;
  });

  println!("这里是必须会执行的，实际场景里，spawn async里面的逻辑不能作为fn的返回: {}", async_ret);
  // println!("noasync fn获取async fn的返回: {}", async_ret.clone());
  // println!("noasync fn获取async fn的返回: {}", async_ret_x);
}

async fn noasync_call_async() {
  println!("---i am async fn---");
}

fn noasync_call_nromal() {
  println!("---i am normal fn---");
}

// todo: 普通函数调用async fn，并且获取async fn的返回
async fn noasync_call_async_res() -> String {
  return "async返回".into();
}

// async fn noasync_call_async_res() -> String {
//   return "async返回".into();
// }



