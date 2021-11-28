// /*
use std::time::SystemTime;
use bincode::serialize;
// use crypto::digest::Digest;
// use crypto::sha2::Sha256;
use super::*;
use easy_hasher::easy_hasher::sha256;
// use crypto::sha2::Sha256;
use serde::{Deserialize, Serialize};
use crate::transaction::Transaction;

const TARGET_HEXS: usize = 4;

#[derive(Debug, Deserialize, Serialize)]
pub struct Block {
  timestamp:  u128,
  // data:   String,
  transactions:  Vec<Transaction>,
  prev_block_hash:  String,
  hash:   String,
  nonce:  i32,
}

impl Block {
   pub fn get_hash(&self) -> String {
    self.hash.clone()
  }

  pub fn get_prev_hash(&self) -> String {
    self.prev_block_hash.clone()
  }

  pub fn get_transaction(&self) -> &Vec<Transaction> {
    &self.transactions
  }

  pub fn new_block(transactions: Vec<Transaction>, prev_block_hash: String) -> Result<Block> {
    let timestamp = SystemTime::now()
      .duration_since(SystemTime::UNIX_EPOCH)?.as_millis();
    let mut block = Block {
      timestamp,
      transactions,
      prev_block_hash,
      hash: "".to_string(),
      nonce: 0
    };
    // block.set_hash()?;
    block.run_proof_work()?;
    Ok(block)
  }

  pub fn new_genesis_block(coinbase: Transaction) -> Block {
    Block::new_block(vec![coinbase],
                     String::new()).unwrap()
  }

  // 执行POW工作证明
  fn run_proof_work(&mut self) -> Result<()> {
    info!("产生交易了，开始挖矿 \"{:#?}\"\n", self.transactions);
    while !self.validate()? {
      self.nonce += 1;
    }
    info!("=== 挖到矿啦 ===");
    let hash_data = self.prepare_hash_data()?;
    self.hash = hash_data;
    Ok(())
  }

  fn prepare_hash_data(&self) -> Result<String> {
    let content = format!("{}{}{}{}{}", self.prev_block_hash.clone(),
      self.transactions.len(), self.timestamp, TARGET_HEXS, self.nonce);
    let hash = sha256(&content);
    let s_hash = hash.to_hex_string();
    Ok(s_hash)
  }

  fn validate(&self) -> Result<bool> {
    let data = self.prepare_hash_data()?;
    let val = data.as_str();
    let data_sp = &val[0..TARGET_HEXS as usize];
    info!("正在挖矿, 匹配特征码为 --- {}", data_sp);
    let check = "0000";
    if data_sp == check {
      return Ok(true);
    }
    Ok(false)
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


