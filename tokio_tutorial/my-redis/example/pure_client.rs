
use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;

fn main() {
  match TcpStream::connect("localhost:9999") {
    Ok(mut stream) => {
      println!("Success connected to server in port 9999");
      let msg = b"hello";
      stream.write(msg).unwrap();
      println!("Send hello to server, wait reply...");

      let mut data = [0 as u8; 6];
      match stream.read_exact(&mut data) {
        Ok(_) => {
          if &data == msg {
            println!("Reply is ok!");
          } else {
            let text = from_utf8(&data).unwrap();
            println!("Unexpected reply: {}", text);
          }
        },

        Err(err) => {
          println!("Failed to receive data: {}", err);
        }
      }
    },

    Err(err) => {
      println!("Failed to connect: {}", err);
    }
  }

  println!("终止socket客户端");
}