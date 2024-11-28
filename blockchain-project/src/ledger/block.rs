use crate::ledger::transaction::Transaction;
use sha2::{Sha256, Digest}; // Cryptographic hash library.

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,               // Position in the chain.
    pub previous_hash: String,    // Hash of the previous block.
    pub transactions: Vec<Transaction>, // Transactions in this block.
    pub merkle_root: String,      // Summary of transactions.
    pub timestamp: u64,           // Time the block was created.
}

impl Block {
    pub fn new(index: u64, previous_hash: String, transactions: Vec<Transaction>, timestamp: u64) -> Self {
        let merkle_root = "mock_merkle_root".to_string(); // Replace with real Merkle root logic later.
        Block {
            index,
            previous_hash,
            transactions,
            merkle_root,
            timestamp,
        }
    }

    pub fn hash(&self) -> String {
        // Combine block fields into a single string for hashing.
        let input = format!(
            "{}{}{}{}",
            self.index, self.previous_hash, self.merkle_root, self.timestamp
        );

        // Hash the input string using SHA-256.
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();

        // Return the hash as a hexadecimal string.
        hex::encode(result)
    }
}

