mod ledger; // Link to the ledger module.

use ledger::{Ledger, Transaction}; // Import components from the ledger.

fn main() {
    // Initialize a new ledger
    let mut ledger = Ledger::new();

    // Create a transaction
    let tx = Transaction::new(
        "Alice".to_string(),
        "Bob".to_string(),
        100,
        "mock_signature".to_string(),
        1234567890,
    );

    // Add a new block containing the transaction
    ledger.create_block(vec![tx], 1234567891);

    // Print the ledger state
    println!("Ledger: {:?}", ledger);
}
