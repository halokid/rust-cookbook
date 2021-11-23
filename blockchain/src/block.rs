/*
use std::time::SystemTime;
use crate::CustomResult;
use bincode::serialize;
// use crypto::digest::Digest;
// use crypto::sha2::Sha256;
use super::*;

pub struct Block {
  timestamp:  u128,
  data:   String,
  prev_block_hash:  String,
  hash:   String,
}

impl Block {
  pub fn set_hash(&mut self) -> CustomResult<()> {
    self.timestamp = SystemTime::now()
      .duration_since(SystemTime::UNIX_EPOCH)?
      .as_millis();
    let content = (self.data.clone(), self.timestamp);
    let bytes = serialize((&content))?;
    let mut hasher = Sha256::new();
    hasher.input(&bytes[..]);
    self.hash = hasher.result_str();

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

 */