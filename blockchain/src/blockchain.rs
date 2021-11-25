use bincode::{deserialize, serialize};
// /*
use super::*;
use crate::block::*;
use sled;

#[derive(Debug)]
pub struct Blockchain {
  // blocks:   Vec<Block>,
  tip:  String,
  current_hash:   String,
  db:   sled::Db,
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
          current_hash: block.get_hash(),
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
          current_hash: lasthash,
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
    self.current_hash = newblock.get_hash();

    Ok(())
  }
}

impl Iterator for Blockchain {
  type Item = Block;

  fn next(&mut self) -> Option<Self::Item> {
    if let Ok(encoded_block)  = self.db.get(&self.current_hash) {
      return match encoded_block {
        None => None,

        Some(b) => {
          if let Ok(block) = deserialize::<Block>(&b) {
            self.current_hash = block.get_prev_hash();
            Some(block)
          } else {
            None
          }
        }
      };
    }
    None
  }
}

 // */




