
mod block;
mod blockchain;
mod cli;
mod transaction;

#[macro_use]
extern crate log;

use crate::cli::{CLi};
use env_logger::Env;

// /*
pub type Result<T> = std::result::Result<T, failure::Error>;

fn main() -> Result<()> {
  env_logger::Builder::from_env(Env::default().default_filter_or("info"))
    .init();

  let mut cli = CLi::new()?;
  cli.run();

  Ok(())
}

 // */


// fn main() {}


