use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream) {
  let mut data = [0 as u8; 50]; // using 50 byte buffer
  while match stream.read(&mut data) {
    Ok(size) => {
      // echo everything!
      stream.write(&data[0..size]).unwrap();
      true
    },

    Err(_) => {
      println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
      stream.shutdown(Shutdown::Both).unwrap();
      false
    }
  } {}      // todo: while {} 可以代替 loop?
}

fn main() {
  let listener = TcpListener::bind("0.0.0.0:9999").unwrap();
  // accept connections and process, spawning a new thread for echo one
  println!("Server listening on port 9999");
  for stream in listener.incoming() {
    match stream {
      Ok(stream) => {
        println!("New connection: {}", stream.peer_addr().unwrap());
        thread::spawn(move || {
          // connection success
          handle_client(stream);
        });
      }

      Err(err) => {
        println!("Error: {}", err);
        // connection fail
      }
    }
  }

  // close the socket server
  drop(listener);
}


