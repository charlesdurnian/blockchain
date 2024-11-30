// src/ledger/streams.rs
use crate::ledger::transaction::Transaction;
use sha2::{Digest, Sha256};

pub struct Stream {
    pub id: usize,
    pub transactions: Vec<Transaction>, // Transactions in the stream
}

impl Stream {
    pub fn new(id: usize) -> Self {
        Stream {
            id,
            transactions: Vec::new(),
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }

    // Generate the Merkle root of transactions in this stream
    pub fn generate_merkle_root(&self) -> String {
        let mut hashes: Vec<String> = self
            .transactions
            .iter()
            .map(|tx| Self::hash_transaction(tx))
            .collect();

        // Combine hashes to calculate the Merkle root
        while hashes.len() > 1 {
            hashes = hashes
                .chunks(2) // Process pairs of hashes
                .map(|pair| {
                    let combined = match pair.len() {
                        2 => format!("{}{}", pair[0], pair[1]), // Combine two hashes
                        1 => format!("{}", pair[0]),           // Single hash (odd count)
                        _ => unreachable!(),
                    };
                    Self::hash_data(&combined)
                })
                .collect();
        }

        // Return the final hash as the Merkle root
        hashes.first().unwrap_or(&"0".to_string()).clone()
    }

    fn hash_transaction(transaction: &Transaction) -> String {
        let data = format!(
            "{}{}{}{}",
            transaction.sender, transaction.receiver, transaction.amount, transaction.timestamp
        );
        Self::hash_data(&data)
    }

    fn hash_data(data: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        let result = hasher.finalize();
        hex::encode(result)
    }
}

