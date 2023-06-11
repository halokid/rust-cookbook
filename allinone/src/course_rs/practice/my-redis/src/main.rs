use std::collections::HashMap;
<<<<<<< HEAD
use std::sync::{Arc, Mutex};
use bytes::Bytes;
use mini_redis::{Command, Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

=======
use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

>>>>>>> d986ec995028e28edc27c3532c29b8f33ad0979e
#[tokio::main]
async fn main() {
  let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

<<<<<<< HEAD
  println!("-->>> Listening");

  let db = Arc::new(Mutex::new(HashMap::new()));

=======
>>>>>>> d986ec995028e28edc27c3532c29b8f33ad0979e
  loop {
    let (socket, _) = listener.accept().await.unwrap();
    // process(socket).await;

<<<<<<< HEAD
    let db = db.clone();

    println!("-->>> Accepted");
    tokio::spawn(async move {
      process(socket, db).await;
=======
    tokio::spawn(async move {
      process(socket).await;
>>>>>>> d986ec995028e28edc27c3532c29b8f33ad0979e
    });
  }
}

/*
async fn process(socket: TcpStream) {
  let mut connection = Connection::new(socket);

  if let Some(frame) = connection.read_frame().await.unwrap() {
    println!("GOT -->>> {:?}", frame);

    let response = Frame::Error("unimplemented".to_string());
    connection.write_frame(&response).await.unwrap();
  }
}
 */

<<<<<<< HEAD
/*
=======
>>>>>>> d986ec995028e28edc27c3532c29b8f33ad0979e
async fn process(socket: TcpStream) {
  use mini_redis::Command::{self, Get, Set};
  use std::collections::HashMap;

  let  mut db = HashMap::new();

  let mut connection = Connection::new(socket);

  while let Some(frame) = connection.read_frame().await.unwrap() {
    let response = match Command::from_frame(frame).unwrap() {
      Set(cmd) => {
        db.insert(cmd.key().to_string(), cmd.value().to_vec());
        Frame::Simple("OK".to_string())
      }

      Get(cmd) => {
        if let Some(value) = db.get(cmd.key()) {
          Frame::Bulk(value.clone().into())
        } else {
          Frame::Null
        }
      }

      cmd => panic!("unimplement {:?}", cmd),
    };

    connection.write_frame(&response).await.unwrap();
  }
}
<<<<<<< HEAD
 */


async fn process(socket: TcpStream, db: Db) {
  use mini_redis::Command::{self, Get, Set};

  let mut connection = Connection::new(socket);

  while let Some(frame) = connection.read_frame().await.unwrap() {
    let response = match Command::from_frame(frame).unwrap() {
      Set(cmd) => {
        let mut db = db.lock().unwrap();
        db.insert(cmd.key().to_string(), cmd.value().clone());
        Frame::Simple("OK".to_string())
      }

      Get(cmd) => {
        let db = db.lock().unwrap();
        if let Some(value) = db.get(cmd.key()) {
          Frame::Bulk(value.clone())
        } else {
          Frame::Null
        }
      }

      cmd => panic!("unimplement {:?}", cmd),
    };

    connection.write_frame(&response).await.unwrap();
  }
}
=======
>>>>>>> d986ec995028e28edc27c3532c29b8f33ad0979e










