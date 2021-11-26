use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::blockchain::Blockchain;
use super::*;

const subsidy: i32 = 10;

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
  // 创建一个新的交易， BTC的交易是用UTOX算法
  pub fn new_UTOX(from: String, amout: i32, bc: &Blockchain) -> Result<Transaction> {
    let mut vin = Vec::new();
    let mut acc_v = (0, HashMap::new());
    acc_v = bc.find_spendable_outputs(from, amout);


  }
}

impl TXInput {
  pub fn can_unlock_output_with(&self, unlocking_data: String) -> bool {
    self.script_sig == unlocking_data
  }
}

impl TXOutput {
  pub fn can_be_unlock_with(&self, unlocking_data: String) -> bool {
    self.script_pub_key == unlocking_data
  }
}




