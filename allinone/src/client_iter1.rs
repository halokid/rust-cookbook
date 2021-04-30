/*
use std::net::TcpStream;
use std::io::{ prelude::*, BufReader, Write };
use std::str;

/*
todo: 程序问题
   必须从第一个网站下载完成后才能从第二个网站进行下载，不符合实际下载情况。
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
  use_server("127.0.0.1", 8080, "发送请求到 127.0.0.1:8080")?;
  use_server("127.0.0.1", 8081, "发送请求到 127.0.0.1:8081")?;

  Ok(())
}

 */

