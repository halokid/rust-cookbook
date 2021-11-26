use std::collections::HashMap;
use bincode::{deserialize, serialize};
// /*
use super::*;
use crate::block::*;
use sled;
use crate::transaction::{Transaction, TXOutput};

#[derive(Debug)]
pub struct Blockchain {
  // blocks:   Vec<Block>,
  tip:  String,
  db:   sled::Db,
}

struct BlockchainIterator<'a> {
  current_hash:   String,
  bc:  &'a Blockchain,
}

impl Blockchain {
  pub fn new() -> Result<Blockchain> {
    info!("=== 建立blockchain实例 ===");
    let db = sled::open("/tmp/blocks")?;

    match db.get("LAST")? {
      // todo: 如果没有最后一个block， 那就要执行创世块
      None => {
        info!("=== 不存在最后一个区块, 创建一个创世块 ===");
        let block = Block::new_genesis_block();
        db.insert(block.get_hash(), serialize(&block)?)?;
        db.insert("LAST", block.get_hash().as_bytes())?;
        let bc = Blockchain {
          tip: block.get_hash(),
          db
        };
        bc.db.flush()?;
        Ok(bc)
      }

      Some(hash) => {
        info!("=== 存在最后一个区块 ===");
        let lasthash = String::from_utf8(hash.to_vec())?;
        Ok(Blockchain{
          tip: lasthash.clone(),
          db
        })
      }
    }
  }

  pub fn add_block(&mut self, data: String) -> Result<()> {
    info!("=== 添加新区块到区块链 ===");
    let lasthash = self.db.get("LAST")?.unwrap();

    let newblock = Block::new_block( data,
           String::from_utf8(lasthash.to_vec())? )?;

    self.db.insert(newblock.get_hash(), serialize(&newblock)?)?;
    self.db.insert("LAST", newblock.get_hash().as_bytes())?;
    self.db.flush()?;

    self.tip = newblock.get_hash();

    Ok(())
  }

  // 返回一个 BlockchainIterator
  pub fn iter(&self) -> BlockchainIterator {
    BlockchainIterator {
      current_hash: self.tip.clone(),
      bc: &self,
    }
  }

  // 返回所有未耗尽的交易输出
  pub fn find_UTXO(&self, address: String) -> Vec<TXOutput> {
    let mut utxos = Vec::<TXOutput>::new();
    let unspend_TXs = self.find_unspent_transactions(address);
    for tx in unspend_TXs {
      for out in tx.vout {
        if out.ca
      }
    }
    utxos
  }

  // 返回一个交易中包含多少可花费的输出
  pub fn find_spendable_outputs(
    &self, address: String, amount: i32
  ) -> (i32, HashMap<String, Vec<i32>>) {
    let mut unspent_output: HashMap<String, Vec<i32>> = HashMap::new();
    let mut accumulated = 0;
    let unspend_TXs = self
  }

  // 返回一个交易中包含多少未耗尽的输出
  pub fn find_unspent_transactions(&self, address: String) -> Vec<&Transaction> {
    let mut spent_TXOs: HashMap<String, Vec<i32>> = HashMap::new();
    let mut unspent_TXs: Vec<Transaction> = Vec::new();

    for block in self.
  }
}

impl<'a> Iterator for BlockchainIterator<'a> {
  type Item = Block;

  fn next(&mut self) -> Option<Self::Item> {
    if let Ok(encoded_block)  = self.db.get(&self.current_hash) {
      return match encoded_block {
        Some(b) => {
          if let Ok(block) = deserialize::<Block>(&b) {
            self.current_hash = block.get_prev_hash();
            Some(block)
          } else {
            None
          }
        },

        None => None,
      };
    }
    None
  }
}

 // */




