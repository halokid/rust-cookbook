use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
pub async fn main() -> tokio::io::Result<()> {
  let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
  println!("Connected to server");

  stream.write_all(b"Hello Server").await?;

  let mut buf = vec![0; 1024];
  let n = stream.read(&mut buf).await?;

  println!("received from server: {}", String::from_utf8_lossy(&buf[..n]));

  Ok(())
}