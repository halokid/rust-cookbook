
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use std::error::Error;

// todo: 这是一个tokio的网络客户端

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
  // open a tcp stream to the socket address
  // not that this is the tokio tcpStream, which is fully async
  let mut stream = TcpStream::connect("127.0.0.1:9527").await?;
  println!("created stream");

  let result = stream.write(b"hello world\n").await;
  println!("wrote to stream; success={:?}", result.is_ok());

  Ok(())
}

