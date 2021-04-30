/*
use std::net::TcpStream;
use std::io::{ prelude::*, BufReader, Write };
use std::str;
use futures::join;
use futures::executor;
use std::thread;
use futures::future::Join;

/*
针对迭代二中的问题，可以考虑用异步编程来解决，即不需要起多个线程，每个任务就绪后再执行，修改过程如下：

todo: 运行程序，发现并没有符合我们的预期，运行结果仍然是先下载第一个任务，再下载第二个任务。为什么会这样呢？
 */

fn use_server(server: &str, port: u16, content: &str) -> std::io::Result<()> {
  let mut stream = TcpStream::connect((server, port))?;
  let _ = stream.write(content.as_bytes())?;

  let mut reader = BufReader::new(&stream);
  let mut buffer: Vec<u8> = Vec::new();
  reader.read_until(b'\n', &mut buffer)?;

  println!("从服务器接收到: {}", str::from_utf8(&buffer).unwrap());
  Ok(())
}

async fn async_use_server(server: &str, port: u16, content: &str) {
  use_server(server, port, content).unwrap();
}

async fn use_all_server() {
  let f1 =- async_use_server("127.0.0.1", 8080, "发送请求到 127.0.0.1:8080");
  let f2 =- async_use_server("127.0.0.1", 8080, "发送请求到 127.0.0.1:8081");

  Join(f1, f2)
}

fn main() -> std::io::Result<()> {
  let f = use_all_server();
  executor::block_on(f);

  Ok(())
}

 */
