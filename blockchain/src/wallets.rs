use super::*;
use bincode::{deserialize, serialize};
use bitcoincash_addr::*;
use crypto::digest::Digest;
use crypto::ed25519;
use crypto::ripemd160::Ripemd160;
use crypto::sha2::Sha256;
use rand::Rng;
use serde::{Deserialize, Serialize};
use sled;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Wallet {
  pub secret_key: Vec<u8>,
  pub public_key: Vec<u8>,
}

impl Wallet {
  fn new() -> Self {
    let mut key: [u8; 64] = [0; 64];
    let mut rand = rand::OsRng::new().unwrap();
    rand.fill_bytes(&mut key);
    let (secret_key, public_key) = ed25519::keypair(&key);
    let secret_key = secret_key.to_vec();
    let public_key = public_key.to_vec();
    Wallet {
      secret_key,
      public_key,
    }
  }
}

