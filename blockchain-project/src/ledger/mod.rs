pub mod transaction; // Link to the transaction module.
pub use transaction::Transaction; // Re-export the Transaction struct for use in other files.

pub mod block; // Link to the block module.
pub use block::Block; // Re-export the Block struct for consistency.

#[derive(Debug)] // Enable debug printing for the Ledger struct.
pub struct Ledger {
    pub blocks: Vec<Block>, // List of blocks in the chain.
}

impl Ledger {
    pub fn new() -> Self {
        // Create a genesis block (the first block in the chain).
        let genesis_block = Block::new(
            0,                      // Index of the genesis block.
            "0".to_string(),        // No previous hash for the genesis block.
            Vec::new(),             // No transactions in the genesis block.
            0                       // Placeholder timestamp.
        );

        Ledger {
            blocks: vec![genesis_block],
        }
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    pub fn create_block(&mut self, transactions: Vec<Transaction>, timestamp: u64) {
        let previous_block = self.blocks.last().unwrap(); // Get the last block.
        let new_block = Block::new(
            self.blocks.len() as u64, // Index of the new block.
            previous_block.hash(),    // Hash of the previous block.
            transactions,
            timestamp,
        );
        self.add_block(new_block);
    }

    pub fn validate_chain(&self) -> bool {
        // Ensure every block links correctly to the previous block.
        for i in 1..self.blocks.len() {
            if self.blocks[i].previous_hash != self.blocks[i - 1].hash() {
                return false;
            }
        }
        true
    }
}

