
mod block;
mod blockchain;

/*
use std::thread::sleep;
use std::time::Duration;
use crate::blockchain::Blockchain;


pub type Result<T> = std::result::Result<T, failure::Error>;

fn main() -> Result<()> {
  let mut bc = Blockchain::new();
  sleep(Duration::from_millis(10));
  bc.add_block(String::from("Send 1 BTC to Halokid"))?;

  sleep(Duration::from_millis(30));
  bc.add_block(String::from("Send 2 more BTC to Halokid"))?;

  println!("Blockchain: {:#?}", bc);
  Ok(())
}

 */


fn main() {}