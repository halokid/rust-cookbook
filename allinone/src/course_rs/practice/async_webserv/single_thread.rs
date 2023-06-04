use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use crossbeam::channel::internal::SelectHandle;
use futures::TryFutureExt;

// single thread version
pub fn single_comm() {
  let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

  println!("-->>> current folder is: {:?}", std::env::current_dir());

  for stream in listener.incoming() {
    let stream = stream.unwrap();

    handle_connect(stream);
  }
}

fn handle_connect(mut stream: TcpStream) {
  let mut buffer = [0; 1024];
  stream.read(&mut buffer).unwrap();

  let get = b"GET / HTTP/1.1\r\n";

  let (status_line, filename) = if buffer.starts_with(get) {
    ("HTTP/1.1 200 OK\r\n\r\n", "./hello.html")
  } else {
    ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "./404.html")
  };

  let content = fs::read_to_string(filename).unwrap();

  let response = format!("{status_line}{content}");
  stream.write_all(response.as_bytes()).unwrap();
  stream.flush().unwrap();
}











