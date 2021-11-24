// /*
use super::*;
use crate::block::*;

#[derive(Debug)]
pub struct Blockchain {
  blocks:   Vec<Block>,
}

impl Blockchain {
  pub fn new() -> Blockchain {
    Blockchain {
      blocks: vec![Block::new_genesis_block()],
    }
  }

  pub fn add_block(&mut self, data: String) -> Result<()> {
    let prev = self.blocks.last().unwrap();
    let newblock = Block::new_block(data, prev.get_hash())?;
    self.blocks.push(newblock);
    Ok(())
  }
}

 // */


