use serde::{Serialize, Deserialize};
use crate::ledger::transaction::Transaction;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub previous_hash: String,
    pub transactions: Vec<Transaction>,
    pub merkle_root: String,
    pub timestamp: u64,
}

impl Block {
    pub fn new(index: u64, previous_hash: String, transactions: Vec<Transaction>, merkle_root: String, timestamp: u64) -> Self {
        Block {
            index,
            previous_hash,
            transactions,
            merkle_root,
            timestamp,
        }
    }
}
