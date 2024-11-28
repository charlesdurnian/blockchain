#[cfg(test)]
mod tests {
    use crate::ledger::{Ledger, Transaction};

    #[test]
    fn test_ledger() {
        let mut ledger = Ledger::new();

        let tx = Transaction::new(
            "Alice".to_string(),
            "Bob".to_string(),
            100,
            "mock_signature".to_string(),
            1234567890,
        );

        ledger.create_block(vec![tx], 1234567891);

        assert!(ledger.validate_chain()); // Ensure the chain is valid.
    }

    #[test]
    fn test_block_hashing() {
        let tx = Transaction::new(
            "Alice".to_string(),
            "Bob".to_string(),
            100,
            "mock_signature".to_string(),
            1234567890,
        );

        let block = crate::ledger::block::Block::new(
            1,
            "0".to_string(),
            vec![tx],
            1234567891,
        );

        assert!(!block.hash().is_empty()); // Ensure the block hash is generated.
    }
}
