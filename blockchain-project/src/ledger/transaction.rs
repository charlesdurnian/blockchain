use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub timestamp: u64,
    pub signature: String,
}

impl Transaction {
    pub fn new(sender: String, receiver: String, amount: u64, signature: String, timestamp: u64) -> Self {
        Transaction {
            sender,
            receiver,
            amount,
            signature,
            timestamp,
        }
    }
}
