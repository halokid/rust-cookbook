// /*
use std::time::SystemTime;
use bincode::serialize;
// use crypto::digest::Digest;
// use crypto::sha2::Sha256;
use super::*;
use easy_hasher::easy_hasher::sha256;

#[derive(Debug)]
pub struct Block {
  timestamp:  u128,
  data:   String,
  prev_block_hash:  String,
  hash:   String,
}

impl Block {
  pub fn set_hash(&mut self) -> Result<()> {
    self.timestamp = SystemTime::now()
      .duration_since(SystemTime::UNIX_EPOCH)?
      .as_millis();
    let data = self.data.clone();
    let timestamp = self.timestamp.to_string();
    let content = format!("{}{}", data, timestamp);
    let hash = sha256(&content);
    let s_hash = hash.to_hex_string();
    self.hash = s_hash;

    Ok(())
  }

  pub fn new_block(data: String, prev_block_hash: String) -> Result<Block> {
    let mut block = Block {
      timestamp: 0,
      data,
      prev_block_hash,
      hash: "".to_string(),
    };
    block.set_hash()?;
    Ok(block)
  }

  pub fn new_genesis_block() -> Block {
    Block::new_block(String::from("Genesis Block"),String::new()).unwrap()
  }

  pub fn get_hash(&self) -> String {
    self.hash.clone()
  }
}

 // */

#[test]
fn set_hash_test() {
  let mut bl = Block{
    timestamp: 0,
    data: "hello".to_string(),
    prev_block_hash: "123456789".to_string(),
    hash: "".to_string()
  };
  bl.set_hash();
  println!("bl -- {:?}", bl);
}


