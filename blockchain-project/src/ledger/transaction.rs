// src/ledger/transaction.rs
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TransactionType {
    Standard,      // Regular transaction (e.g., payment)
    SmartContract, // Deploy or execute a smart contract
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,                 // For Standard transactions
    pub timestamp: u64,              // Original timestamp
    pub verified_timestamp: u64,     // Verified timestamp (VDF)
    pub signature: String,
    pub tx_type: TransactionType,    // Type of transaction
    pub contract_code: Option<Vec<u8>>, // Smart contract bytecode (if applicable)
    pub contract_data: Option<String>,  // Input data for contract execution (if applicable)
}

impl Transaction {
    pub fn new(
        sender: String,
        receiver: String,
        amount: u64,
        signature: String,
        timestamp: u64,
        tx_type: TransactionType,
        contract_code: Option<Vec<u8>>,
        contract_data: Option<String>,
    ) -> Self {
        Transaction {
            sender,
            receiver,
            amount,
            timestamp,
            verified_timestamp: 0,
            signature,
            tx_type,
            contract_code,
            contract_data,
        }
    }
}

