use std::net::TcpStream;
use std::io::{ prelude::*, BufReader, Write };
use std::str;
use std::thread;

/*
使用多线程实现，每个下载任务都起一个线程。

todo: 程序存在问题
1、如果有几千几万个任务，每个任务都起一个线程，怎么办？
2、如果使用线程池解决，但是我们每个下载任务其实都是发起请求后在等待server的响应，效率还是不够高。那么有没有什么办法解决这些问题？
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

fn main() -> std::io::Result<()> {
  let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();

  let handle = thread::spawn(move || {
    use_server("127.0.0.1", 8080, "发送请求到 127.0.0.1:8080").unwrap();
  });
  handles.push(handle);

  let handle = thread::spawn(move || {
    use_server("127.0.0.1", 8081, "发送请求到 127.0.0.1:8081").unwrap();
  });
  handles.push(handle);

  for handle in handles {
    handle.join().unwrap();
  }

  Ok(())
}


