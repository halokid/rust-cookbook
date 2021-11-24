// /*
use std::time::SystemTime;
use bincode::serialize;
// use crypto::digest::Digest;
// use crypto::sha2::Sha256;
use super::*;
use easy_hasher::easy_hasher::sha256;
// use crypto::sha2::Sha256;
use serde::{Deserialize, Serialize};

const TARGET_HEXS: usize = 4;

#[derive(Debug, Deserialize, Serialize)]
pub struct Block {
  timestamp:  u128,
  data:   String,
  prev_block_hash:  String,
  hash:   String,
  nonce:  i32,
}

impl Block {
  /*
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
   */

  pub fn new_block(data: String, prev_block_hash: String) -> Result<Block> {
    let timestamp = SystemTime::now()
      .duration_since(SystemTime::UNIX_EPOCH)?.as_millis();
    let mut block = Block {
      timestamp,
      data,
      prev_block_hash,
      hash: "".to_string(),
      nonce: 0
    };
    // block.set_hash()?;
    block.run_proof_work()?;
    Ok(block)
  }

  pub fn new_genesis_block() -> Block {
    Block::new_block(String::from("Genesis Block"),String::new()).unwrap()
  }

  pub fn get_hash(&self) -> String {
    self.hash.clone()
  }

  pub fn get_prev_hash(&self) -> String {
    self.prev_block_hash.clone()
  }

  /*
  fn prepare_hash_data(&self) -> Result<Vec<u8>> {
    let content = (
      self.prev_block_hash.clone(),
      self.data.clone(),
      self.timestamp,
      TARGET_HEXS,
      self.nonce,
    );
    let bytes = serialize(&content)?;
    Ok(bytes)
  }
   */

  fn prepare_hash_data(&self) -> Result<String> {
    let content = format!("{}{}{}{}{}", self.prev_block_hash.clone(),
      self.data.clone(), self.timestamp, TARGET_HEXS, self.nonce);
    let hash = sha256(&content);
    let s_hash = hash.to_hex_string();
    Ok(s_hash)
  }

  fn validate(&self) -> Result<bool> {
    let data = self.prepare_hash_data()?;
    let val = data.as_str();
    let data_sp = &val[0..TARGET_HEXS as usize];
    println!("data_sp --- {}", data_sp);
    let check = "0000";
    if data_sp == check {
      return Ok(true);
    }
    Ok(false)
  }

  fn run_proof_work(&mut self) -> Result<()> {
    println!("Mining the block containing \"{}\"\n", self.data);
    while !self.validate()? {
      self.nonce += 1;
    }
    let hash_data = self.prepare_hash_data()?;
    self.hash = hash_data;
    Ok(())
  }
}

 // */

/*
#[test]
fn set_hash_test() {
  let mut bl = Block{
    timestamp: 0,
    data: "hello".to_string(),
    prev_block_hash: "123456789".to_string(),
    hash: "".to_string(),
    nonce: 0
  };
  bl.set_hash();
  println!("bl -- {:?}", bl);
}
 */

#[test]
fn validate_test() {
  let mut bl = Block::new_block("i am test block".to_string(),
                                "1234567".to_string()).unwrap();
  for i in 0..1000000 {
    println!("i -- {}", i);
    let check = bl.validate().unwrap();
    if !check {
      bl.nonce += 1;
    } else {
      println!("计算出了，挖到矿啦");
      break
    }
  }
}


