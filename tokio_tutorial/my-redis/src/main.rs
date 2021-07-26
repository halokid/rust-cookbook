use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};
use tokio::task;
use std::rc::Rc;
use tokio::task::yield_now;
use mini_redis::Command::{self, Get, Set};
use std::collections::HashMap;
use std::option::Option::Some;

use bytes::Bytes;
use std::sync::{ Arc, Mutex };
use tokio::sync::mpsc;

// todo： 全局声明变量
type Db  = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {

  // +++++++++++++++++++++ Tutorial 02 +++++++++++++++++++++++
  tokio::spawn(async {
    // The scope forces `rc` to drop before `.await`.
    {
      let rc = Rc::new("hello");
      // yield_now().await;
      println!("rc: {}", rc);
    }

    // `rc` is no longer used. It is **not** persisted when
    // the task yields to the scheduler
    yield_now().await;
  });


  // +++++++++++++++++++++ Tutorial 01 +++++++++++++++++++++++
  /*
  // todo: --------------- 一些试验代码 ----------------
>>>>>>> 4ca8b25eecb1c4242f143fd6cc583841c9ed145b
  // todo: spawn 返回值
  // todo: 有些传递的 async 语句块是具有返回值的，调用者通过 JoinHandle 的 .await 来获取其返回值，
  let handle = tokio::spawn(async {
    "spawn返回的值"
  });
  let out = handle.await.unwrap();
  println!("tokio spawn返回值: {}", out);

  // todo: spawn怎么引用 外部线程的变量
  let v = vec![1, 2, 3];
  task::spawn(async move {    //todo: 这个和 tokio::spawn是一样的
    println!("task spawn");
    println!("v -------- {:?}", v);
  });
   */

  // todo: -------------- 监听网络连接 --------------------
  // Create a new channel with a capacity of at most 32
  let (tx, mut rx) = mpsc::channel(32);
  let tx2 = tx.clone();

  tokio::spawn( async move {
    tx.send("从 tx handle发送的数据").await;
  });

  tokio::spawn( async move {
    tx.send("从 tx2 handle发送的数据").await;
  });

  let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

  // todo: -------------- 非并发处理 -----------------------
  // loop {
  //   let (socket, _) = listener.accept().await.unwrap();
  //   process(socket).await;
  // }

  // todo: -------------- 并发处理， 事实上，Tokio 能够在单线程中并发运行非常多的任务 -------------
  /*
  loop {
    let (socket, _) = listener.accept().await.unwrap();
    // A new task is spqwned for each inbound socket. the socket is
    // moved to the new task and processed there.
    tokio::spawn(async move {
      process(socket).await;
    });
  }
<<<<<<< HEAD
=======
   */

  // todo: -------------- 并发处理，在多个线程中共用存储db -------------
  println!("---正在监听服务---");
  let db = Arc::new(Mutex::new(HashMap::new()));
  loop {
    let (socket, _) = listener.accept().await.unwrap();
    let db = db.clone();
    println!("---正在接收连接---");
    tokio::spawn(async move {
      process(socket, db).await;
    });
  }
}

/*
async fn process(socket: TcpStream) {
  let mut connection = Connection::new(socket);
  if let Some(frame) = connection.read_frame().await.unwrap() {
    println!("获得redis数据frame: {:?}", frame);

    // 假如服务端 response是一个error
    let response = Frame::Error("未执行".to_string());
    connection.write_frame(&response);
  }
}
 */

// todo: 需要增强的点是, db还没有在不同的连接中共享， 如果其他的客户端连接要使用get或者hello的值，将获取不到数据
/*
>>>>>>> 4ca8b25eecb1c4242f143fd6cc583841c9ed145b
async fn process(socket: TcpStream) {
  let mut db = HashMap::new();
  let mut connection = Connection::new(socket);

  while let Some(frame) = connection.read_frame().await.unwrap() {
    let response = match Command::from_frame(frame).unwrap() {
      Set(cmd) => {
<<<<<<< HEAD
        // todo: HashMap的 insert操作要先放上来， 用来声明HashMap的 k, v 的变量类型
=======
>>>>>>> 4ca8b25eecb1c4242f143fd6cc583841c9ed145b
        db.insert(cmd.key().to_string(), cmd.value().to_vec());
        Frame::Simple("OK".to_string())
      },
      Get(cmd) => {
        if let Some(value) = db.get(cmd.key()) {
          Frame::Bulk(value.clone().into())
        } else {
          Frame::Null
        }
      },
<<<<<<< HEAD
      cmd => panic!("没有执行命令 {:?}", cmd),
=======
      cmd  => panic!("未执行 {:?}", cmd),
>>>>>>> 4ca8b25eecb1c4242f143fd6cc583841c9ed145b
    };

    connection.write_frame(&response).await.unwrap();
  }
<<<<<<< HEAD
}








=======

}
 */

// todo: Tutorial 4, Channel通道


// todo: Tutorial 3, 线程间共享存储
async fn process(socket: TcpStream, db: Db) {
  // let mut db = HashMap::new();
  let mut connection = Connection::new(socket);

  while let Some(frame) = connection.read_frame().await.unwrap() {
    let response = match Command::from_frame(frame).unwrap() {
      Set(cmd) => {
        let mut db = db.lock().unwrap();
        db.insert(cmd.key().to_string(), cmd.value().clone());
        Frame::Simple("OK".to_string())
      },

      Get(cmd) => {
        let db = db.lock().unwrap();
        if let Some(value) = db.get(cmd.key()) {
          Frame::Bulk(value.clone().into())
        } else {
          Frame::Null
        }
      },
      cmd  => panic!("未执行 {:?}", cmd),
    };

    connection.write_frame(&response).await.unwrap();
  }

}


