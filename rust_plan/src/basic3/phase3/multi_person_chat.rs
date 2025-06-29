use tokio::{
  io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
  net::{TcpListener, TcpStream},
  sync::broadcast,
};

pub async fn c1() -> tokio::io::Result<()> {
  let listener = TcpListener::bind("127.0.0.1:8080").await?;
  println!("Chat server running on 127.0.0.1:8080");


  // broadcast channel, sender and receiver
  let (tx, _rx) = broadcast::channel::<String>(100);

  loop {
    let (socket, addr) = listener.accept().await?;
    println!("New Client: {}", addr);

    // each connect has it's own tx, rx
    let tx = tx.clone();
    let mut rx = tx.subscribe();

    tokio::spawn(async move {
      let (reader, mut writer) = socket.into_split();
      let mut reader = BufReader::new(reader);
      let mut line = String::new();

      loop {
        tokio::select! {
          // client input
          result = reader.read_line(&mut line) => {
            match result {
              Ok(0) => break,

              Ok(_) => {
                let msg = format!("{} say: {}", addr, line.trim());
                if tx.send(msg).is_err() {
                  break;
                }
                line.clear();
              }

              Err(_) => break,
            }
          }

          // come from boardcast message
          result = rx.recv() => {
            match result {
              Ok(msg) => {
                if writer.write_all(msg.as_bytes()).await.is_err() {
                  break;
                }

                if writer.write_all(b"\n").await.is_err() {
                  break;
                }
              }

              Err(_) => break,
            }
          }
        }
      }

      println!("Client {} disconnected", addr);
    });
  }
}


