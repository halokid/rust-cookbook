
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use std::io::Error;

#[tokio::main]
fn main() -> io::Result<()> {
  let listener = TcpListener::bind("127.0.0.1:6142").await.unwrap();

  loop {
    let (mut socket, _) = listener.accept().await?;

    tokio::spawn(async move {
      let mut buf = vec![0; 1024];

      loop {
        match socket.read(&mut buf).await {
          //  // Return value of `Ok(0)` signifies that the remote has closed
          Ok(0) => return,

          Ok(n) => {
            // copy the data back to socket
            if socket.write_all(&buf[..n]).await.is_err() {
              // 意外的socket错误， 停止处理
              return;
            }
          },

          Err(_) => {
            // 意外的socket错误， 停止处理
            return;
          },
        }
      }
    });
  }
}