// todo: 异步场景9： 一个公用的数据, 起一个线程去检查循环定时修改，然后主线程可以获取最新的修改数据, 主线程是一个async函数, 主线程获取到数据最后， 再去更新主线程上的生命周期的变量


use std::option::Option::Some;
use std::thread::sleep;
use tokio::time::Duration;
use tokio::sync::mpsc;

// todo: 错误范例
/*
#[tokio::main]
pub async fn comm() {
  let mut v = "foo".to_string();
  // tokio::task::spawn(async {
    // for i in 0..99 {
    //   v =  format!("{}-{}", v, i.to_string());
    // }
    // println!("---在spawn里面 1---");
  // });

  tokio::task::spawn(async move {
  // tokio::task::spawn(async {    // todo: 编译错误, ^ may outlive borrowed value `v`
    for i in 0..30 {
      v =  format!("{}-{}", v, i.to_string());
      println!("v ------ {}", v);
    }
    println!("---在spawn里面 2---");
  });

  sleep(Duration::from_secs(5));
  // todo: 编译错误， ^ value borrowed here after move
  // println!("主线程的 v --------- {}", v.clone());
  println!("---comm完成---");
}
 */

#[tokio::main]
pub async fn comm() {
  let (tx, mut rx) = mpsc::channel(1);
  let mut v1 = "foo".to_string();

  tokio::task::spawn(async move {
    for i in 0..9 {
      // let v =  format!("{}-{}", v1, i.to_string());
      // println!("v ------ {}", v);
      println!("i ------ {}", i);
      tx.send(i).await.unwrap();
      sleep(Duration::from_secs(2));
    }
  });

  while let Some(ix) = rx.recv().await {
    println!("async主线程接收到的变化值 --- {:?}", ix);
    print_share_var(ix);
  }
}

#[tokio::main]
pub async fn comm2() -> String {
  return "tokio可以在noasync函数里面返回吗？".to_string();
}


fn print_share_var(ix: i32) {
  // println!("共享的数据为 ---- {}", ix);
  if ix % 2 == 0 {
    println!("接收ix变化为偶数 --- {}", ix);
  } else {
    println!("接收ix变化为奇数 --- {}", ix);
  }
}






