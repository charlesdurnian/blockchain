use crate::ledger::transaction::Transaction;

pub struct Stream {
    pub id: usize,
    pub transactions: Vec<Transaction>,
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

    pub fn generate_merkle_root(&self) -> String {
        // Placeholder for Merkle root calculation
        "dummy_merkle_root".to_string()
    }
}
