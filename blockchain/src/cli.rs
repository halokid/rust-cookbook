use super::*;
use crate::blockchain::*;
use clap::{App, Arg};

pub struct CLi {
  bc: Blockchain,
}

impl CLi {
  pub fn new() -> Result<CLi> {
    Ok(CLi{
      bc: Blockchain::new()?,
    })
  }

  pub fn run(&mut self) -> Result<()> {
    info!("=== 运行区块链,请输入参数 ===");
    let matches = App::new("blockchain-demo")
      .version("0.1")
      .author("halokid")
      .about("reimplement blockchain_go in rust: a simple blockchain for learning")
      .subcommand(App::new("printchain").about("print all the chain blocks"))
      .subcommand(
        App::new("addblock")
          .about("add a block in the blockchain")
          .arg(Arg::from_usage("<data> 'the blockchain data'")),
      )
      .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("addblock") {
      if let Some(c) = matches.value_of("data") {
        self.addblock(String::from(c))?;
      } else {
        println!("Not printing testing lists...");
      }
    }

    if let Some(_) = matches.subcommand_matches("printchain") {
      self.print_chain();
    }

    Ok(())
  }

  fn print_chain(&mut self) {
    let mut i = 0;
    for b in &mut self.bc {
      println!("区块{}: {:#?}", i, b);
      i += 1;
    }
  }

  fn addblock(&mut self, data: String) -> Result<()> {
    self.bc.add_block(data)
  }
}




