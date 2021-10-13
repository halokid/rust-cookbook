use std::error::Error;
use std::env;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:9527".to_string());
  let listener = TcpListener::bind(&addr).await?;
  println!("Listening on: {}", addr);

  loop {
    let (mut socket, _) = listener.accept().await?;
    tokio::spawn(async move {
      let mut buf = vec![0; 1024];

      // in a loop, read data from socket and write the data back
      /*
      // todo: 下面这样写回处理两次client发送来的消息???
      loop {
        let n = socket.read(&mut buf).await.expect("failed to read data from socket");
        println!("read data from client: {:?}", buf);
        let data_s = String::from_utf8(buf.clone());
        println!("data_s: {}", data_s.unwrap());

        if n == 0 {
          return ;
        }

        socket.write_all(&buf[0..n]).await.expect("failed to write data to socket")
      }
       */


      let n = socket.read(&mut buf).await.expect("failed to read data from socket");
      println!("read data from client: {:?}", buf);
      let data_s = String::from_utf8(buf.clone());
      println!("data_s: {}", data_s.unwrap());

      if n == 0 {
        return;
      }

      socket.write_all(&buf[0..n]).await.expect("failed to write data to socket")

    });
  }
}


