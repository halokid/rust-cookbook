use std::collections::HashMap;
use bincode::{deserialize, serialize};
// /*
use super::*;
use crate::block::*;
use sled;
use crate::transaction::{Transaction, TXOutput};

const GENESIS_COINBASE_DATA: &str = "=== blockchain发行了部分数字资产 ===";

#[derive(Debug)]
pub struct Blockchain {
  // blocks:   Vec<Block>,
  tip: String,
  db: sled::Db,
}

pub struct BlockchainIterator<'a> {
  current_hash: String,
  bc: &'a Blockchain,
}

impl Blockchain {
  /// NewBlockchain creates a new Blockchain db
  pub fn new() -> Result<Blockchain> {
    info!("=== 建立blockchain实例 ===");

    let db = sled::open("data/blocks")?;
    let hash = db.get("LAST")?
      .expect("=== 必须首先创建一个区块数据 ===");
    info!("=== 找到区块数据 ===");
    let lasthash = String::from_utf8(hash.to_vec())?;
    Ok(Blockchain {
      tip: lasthash.clone(),
      db,
    })
  }

  /// CreateBlockchain creates a new blockchain DB
  pub fn create_blockchain(address: String) -> Result<Blockchain> {
    info!("=== 创建新的区块链blockchain ===");

    let db = sled::open("data/blocks")?;
    info!("=== 在创建新区块数据 ===");
    let cbtx = Transaction::new_coinbase(address,
       String::from(GENESIS_COINBASE_DATA))?;
    let genesis: Block = Block::new_genesis_block(cbtx);

    db.insert(genesis.get_hash(), serialize(&genesis)?)?;
    db.insert("LAST", genesis.get_hash().as_bytes())?;
    let bc = Blockchain {
      tip: genesis.get_hash(),
      db,
    };
    bc.db.flush()?;
    Ok(bc)
  }

  /// MineBlock mines a new block with the provided transactions
  pub fn mine_block(&mut self, transactions: Vec<Transaction>) -> Result<()> {
    info!("=== 添加新区块到区块链 ===");
    let lasthash = self.db.get("LAST")?.unwrap();

    let newblock = Block::new_block(transactions,
                                    String::from_utf8(lasthash.to_vec())?)?;

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

  /// 返回所有没有完成的交易总共有多少输出
  pub fn find_UTXO(&self, address: String) -> Vec<TXOutput> {
    let mut utxos = Vec::<TXOutput>::new();
    let addressx = address.clone();
    let unspend_TXs = self.find_unspent_transactions(address);

    for tx in unspend_TXs {
      for out in tx.vout {
        if out.can_be_unlock_with(addressx.clone()) {
          utxos.push(out)
        }
      }
    }
    utxos
  }

  // 返回所有有效的交易包含多少可花费的输出
  // @param: amount, 要交易的金额
  pub fn find_spendable_outputs(
    &self, address: String, amount: i32,
  ) -> (i32, HashMap<String, Vec<i32>>) {
    let mut unspent_outputs: HashMap<String, Vec<i32>> = HashMap::new();
    let mut accumulated = 0;
    let addressx = address.clone();
    let unspend_TXs = self.find_unspent_transactions(address);

    for tx in unspend_TXs {
      for index in 0..tx.vout.len() {
        if tx.vout[index].can_be_unlock_with(addressx.clone())
          && accumulated < amount {
          match unspent_outputs.get(&tx.id) {
            None => {
              unspent_outputs.insert(tx.clone().id, vec![index as i32]);
            }

            // Some(mut v) => { v.push(index as i32) }
            _ => {}
          }
          accumulated += tx.vout[index].value;

          if accumulated >= amount {
            return (accumulated, unspent_outputs);
          }
        }
      }
    }

    (accumulated, unspent_outputs)
  }

  // 返回还有多少未完成的交易
  pub fn find_unspent_transactions(&self, address: String) -> Vec<Transaction> {
    let mut spent_TXOs: HashMap<String, Vec<i32>> = HashMap::new();
    let mut unspent_TXs: Vec<Transaction> = Vec::new();

    for block in self.iter() {
      for tx in block.get_transaction() {
        for index in 0..tx.vout.len() {
          if let Some(ids) = spent_TXOs.get(&tx.id) {
            if ids.contains(&(index as i32)) {
              continue;
            }
          }

          if tx.vout[index].can_be_unlock_with(address.clone()) {
            unspent_TXs.push(tx.to_owned());
          }
        }

        if !tx.is_coinbase() {
          for i in &tx.vin {
            if i.can_unlock_output_with(address.clone()) {
              match spent_TXOs.get(&i.txid) {
                None => {
                  spent_TXOs.insert(i.txid.clone(), vec![i.vout]);
                }
                // Some(mut v) => {
                //   v.push(i.vout)
                // }
                _ => {}
              }
            }
          }
        }
      }
    }

    unspent_TXs
  }
}

impl<'a> Iterator for BlockchainIterator<'a> {
  type Item = Block;

  fn next(&mut self) -> Option<Self::Item> {
    if let Ok(encoded_block) = self.bc.db.get(&self.current_hash) {
      return match encoded_block {
        Some(b) => {
          if let Ok(block) = deserialize::<Block>(&b) {
            self.current_hash = block.get_prev_hash();
            Some(block)
          } else {
            None
          }
        }
        None => None,
      };
    }
    None
  }
}

// */




