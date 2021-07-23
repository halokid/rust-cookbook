use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};
use tokio::task;

#[tokio::main]
async fn main() {

  // todo: spawn 返回值
  // todo: 有些传递的 async 语句块是具有返回值的，调用者通过 JoinHandle 的 .await 来获取其返回值，
  let handle = tokio::spawn(async {
    "spawn返回的值"
  });
  let out = handle.await.unwrap();
  println!("tokio spawn返回值: {}", out);

  // todo: spawn怎么引用 外部线程的变量
  let v = vec![1, 2, 3];
  task::spawn(async move {
    println!("task spawn");
    println!("v -------- {:?}", v);
  });

  // todo: -------------- 并发处理网络连接 --------------------
  let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

  // todo: 非并发处理
  // loop {
  //   let (socket, _) = listener.accept().await.unwrap();
  //   process(socket).await;
  // }

  // todo: 并发处理， 事实上，Tokio 能够在单线程中并发运行非常多的任务
  loop {
    let (socket, _) = listener.accept().await.unwrap();
    // A new task is spqwned for each inbound socket. the socket is
    // moved to the new task and processed there.
    tokio::spawn(async move{
      process(socket).await;
    });
  }

}

async fn process(socket: TcpStream) {
  let mut connection = Connection::new(socket);
  if let Some(frame) = connection.read_frame().await.unwrap() {
    println!("获得redis数据frame: {:?}", frame);

    // 假如服务端 response是一个error
    let response = Frame::Error("未执行".to_string());
    connection.write_frame(&response);
  }
}

