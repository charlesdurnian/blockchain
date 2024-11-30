//src/ledger/block.rs
use crate::ledger::transaction::Transaction;
use sha2::{Sha256, Digest};

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,               // Position in the chain
    pub previous_hash: String,    // Hash of the previous block
    pub transactions: Vec<Transaction>, // Transactions in this block
    pub merkle_root: String,      // Summary of transactions
    pub timestamp: u64,           // Time the block was created
}

impl Block {
    pub fn new(index: u64, previous_hash: String, transactions: Vec<Transaction>, timestamp: u64) -> Self {
        let merkle_root = Self::calculate_merkle_root(&transactions);
        Block {
            index,
            previous_hash,
            transactions,
            merkle_root,
            timestamp,
        }
    }

    pub fn hash(&self) -> String {
        let input = format!(
            "{}{}{}{}",
            self.index, self.previous_hash, self.merkle_root, self.timestamp
        );
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        hex::encode(hasher.finalize())
    }

    fn calculate_merkle_root(transactions: &[Transaction]) -> String {
        if transactions.is_empty() {
            return "0".to_string();
        }

        let mut hashes: Vec<String> = transactions
            .iter()
            .map(|tx| {
                let tx_data = format!(
                    "{}{}{}{}",
                    tx.sender, tx.receiver, tx.amount, tx.timestamp
                );
                let mut hasher = Sha256::new();
                hasher.update(tx_data.as_bytes());
                hex::encode(hasher.finalize())
            })
            .collect();

        while hashes.len() > 1 {
            hashes = hashes
                .chunks(2)
                .map(|pair| {
                    let combined = if pair.len() == 2 {
                        format!("{}{}", pair[0], pair[1])
                    } else {
                        pair[0].clone()
                    };
                    let mut hasher = Sha256::new();
                    hasher.update(combined.as_bytes());
                    hex::encode(hasher.finalize())
                })
                .collect();
        }

        hashes[0].clone()
    }
}


