use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::blockchain::Blockchain;
use super::*;
use easy_hasher::easy_hasher::sha256;

const SUBSIDY: i32 = 10;

// 代表一次交易的输入
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TXInput {
  pub txid:   String,
  pub vout:   i32,
  pub script_sig:   String,
}

// 代表一次交易的输出
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TXOutput {
  pub value:  i32,
  pub script_pub_key:   String,
}

// 代表一次交易
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
  pub id: String,
  pub vin:  Vec<TXInput>,
  pub vout:   Vec<TXOutput>,
}

impl Transaction {
  // 创建一个新的交易， BTC的交易是用UTXO算法
  pub fn new_UTXO(from: &str, to: &str, amout: i32, bc: &Blockchain) -> Result<Transaction> {
    let mut vin = Vec::new();
    let acc_v = bc.find_spendable_outputs(
      from.to_string(),amout);

    for tx in acc_v.1 {
      for out in tx.1 {
        let input = TXInput {
          txid: tx.0.clone(),
          vout: out,
          script_sig: from.to_string(),
        };
        vin.push(input);
      }
    }

    let mut vout = vec![TXOutput {
      value: amout,
      script_pub_key: to.to_string()
    }];
    if acc_v.0 > amout {
      vout.push(TXOutput {
        value: acc_v.0 - amout,
        script_pub_key: from.to_string()
      })
    }

    let mut tx = Transaction {
      id: "".to_string(),
      vin,
      vout
    };
    tx.set_id()?;
    Ok(tx)
  }

   /// NewCoinbaseTX creates a new coinbase transaction
   pub fn new_coinbase(to: String, mut data: String) -> Result<Transaction> {
     if data == String::from("") {
       data += &format!("奖励给 {}", to);
     }
     let mut tx = Transaction {
       id: "".to_string(),
       vin: vec![TXInput {
         txid: "".to_string(),
         vout: -1,
         script_sig: data
       }],

       vout: vec![TXOutput {
         value: SUBSIDY,
         script_pub_key: to
       }]
     };

     tx.set_id()?;
     Ok(tx)
   }

  /// SetID sets ID of a transaction
  fn set_id(&mut self) -> Result<()> {
    let content = format!("{:#?}", self);
    let hash = sha256(&content);
    let s_hash = hash.to_hex_string();
    self.id = s_hash;
    Ok(())
  }

  /// IsCoinbase checks whether the transaction is coinbase
  pub fn is_coinbase(&self) -> bool {
    self.vin.len() == 1 && self.vin[0].txid.is_empty()
    && self.vin[0].vout == -1
  }
}

impl TXInput {
  /// CanUnlockOutputWith checks whether the address initiated the transaction
  pub fn can_unlock_output_with(&self, unlocking_data: String) -> bool {
    self.script_sig == unlocking_data
  }
}

impl TXOutput {
  /// CanBeUnlockedWith checks if the output can be unlocked with the provided data
  pub fn can_be_unlock_with(&self, unlocking_data: String) -> bool {
    self.script_pub_key == unlocking_data
  }
}




