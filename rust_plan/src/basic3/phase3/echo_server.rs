use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn c1() -> tokio::io::Result<()> {
  let listener = TcpListener::bind("127.0.0.1:8080").await?;
  println!("Server running on 127.0.0.1:8080");

  loop {
    let (mut socket, addr) = listener.accept().await?;
    println!("New client: {}", addr);

    tokio::spawn(async move {
      let mut buf = [0; 1024];

      loop {
        match socket.read(&mut buf).await {
          Ok(0) => {
            println!("Client {} disconnected", addr);
            break;
          }
          Ok(n) => {
            let received = String::from_utf8_lossy(&buf[..n]);
            let uppercased = received.to_uppercase();

            // if let Err(e) = socket.write_all(&buf[0..n]).await {
            if let Err(e) = socket.write_all(uppercased.as_bytes()).await {
              println!("Fail to write to client {}: {:?}", addr, e);
              break
            }
          }
          Err(e) => {
            println!("Failed to read from client {}: {:?}", addr, e);
            break;
          }
        }
      }
    });
  }
}