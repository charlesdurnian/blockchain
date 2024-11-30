// src/ledger/mod.rs
pub mod transaction;
pub mod streams;
pub mod state;
pub mod block;
pub mod contract; // Add the contract module

pub use transaction::{Transaction, TransactionType};
pub use block::Block;
pub use contract::SmartContract;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Ledger {
    pub blocks: Vec<Block>,                    // List of all blocks
    pub max_transactions: usize,               // Maximum transactions per block
    pub contracts: HashMap<String, SmartContract>, // Deployed contracts
}

impl Ledger {
    pub fn new(max_transactions: usize) -> Self {
        let genesis_block = Block::new(
            0,
            "0".to_string(),
            Vec::new(),
            0,
        );

        Ledger {
            blocks: vec![genesis_block],
            max_transactions,
            contracts: HashMap::new(),
        }
    }

    pub fn process_transaction(&mut self, transaction: Transaction) {
        match transaction.tx_type {
            TransactionType::Standard => {
                println!("Processing standard transaction: {:?}", transaction);
            }
            TransactionType::SmartContract => {
                if let Some(code) = transaction.contract_code {
                    println!("Deploying smart contract...");
                    let contract = SmartContract::new(code);
                    self.contracts.insert(transaction.receiver.clone(), contract);
                }
                if let Some(input) = transaction.contract_data {
                    if let Some(contract) = self.contracts.get_mut(&transaction.receiver) {
                        let result = contract.execute(input);
                        println!("Contract Execution Result: {}", result);
                    }
                }
            }
        }
    }
}
