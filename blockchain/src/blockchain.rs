use bincode::serialize;
// /*
use super::*;
use crate::block::*;
use sled;
use sled::IVec;

#[derive(Debug)]
pub struct Blockchain {
  // blocks:   Vec<Block>,
  tip:  String,
  current_hash:   String,
  db:   sled::Db,
}

impl Blockchain {
  pub fn new() -> Result<Blockchain> {
    let db = sled::open("/tmp/blocks")?;

    match db.get("LAST")? {
      // todo: 如果没有最后一个block， 那就要执行创世块
      None => {
        let block = Block::new_genesis_block();
        db.insert(block.get_hash(), serialize(&block))?;
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
    // let prev = self.blocks.last().unwrap();
    // let newblock = Block::new_block(data, prev.get_hash())?;
    // self.blocks.push(newblock);
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
  type Item = ();

  fn next(&mut self) -> Option<Self::Item> {

  }
}

 // */




