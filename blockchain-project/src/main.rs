// src/main.rs
mod ledger;

use ledger::{Ledger, Transaction, TransactionType};

fn main() {
    // Step 1: Create a ledger
    let mut ledger = Ledger::new(2);

    // Step 2: Deploy a smart contract
    let contract_code = vec![0x01, 0x02, 0x03]; // Placeholder bytecode
    let deploy_tx = Transaction::new(
        "Alice".to_string(),
        "Contract1".to_string(),
        0,
        "mock_signature".to_string(),
        1234567890,
        TransactionType::SmartContract,
        Some(contract_code),
        None,
    );
    ledger.process_transaction(deploy_tx);

    // Step 3: Execute the smart contract
    let execute_tx = Transaction::new(
        "Bob".to_string(),
        "Contract1".to_string(),
        0,
        "mock_signature".to_string(),
        1234567891,
        TransactionType::SmartContract,
        None,
        Some("Hello, Contract!".to_string()),
    );
    ledger.process_transaction(execute_tx);

    // Step 4: Print the ledger state
    println!("{:?}", ledger);
}