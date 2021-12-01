#![allow(non_snake_case)]
mod block;
mod blockchain;
mod cli;
mod transaction;
mod wallets;

#[macro_use]
extern crate log;

pub type Result<T> = std::result::Result<T, failure::Error>;

use crate::cli::Cli;
use env_logger::Env;

fn main() -> Result<()> {
    env_logger::from_env(Env::default().default_filter_or("warning")).init();

    let mut cli = Cli::new()?;
    cli.run()?;

    Ok(())
}




