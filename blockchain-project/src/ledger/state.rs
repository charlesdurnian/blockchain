// src/ledger/state.rs
use crate::ledger::streams::Stream;
use crate::ledger::transaction::Transaction;

pub struct StateManager {
    pub streams: Vec<Stream>, // All the streams
}

impl StateManager {
    pub fn apply_vdf(&self, input: u64) -> u64 {
        // Simulate computational delay (placeholder for a real VDF)
        let mut result = input;
        for _ in 0..10_000_000 {
            result = result.wrapping_mul(31).wrapping_add(7);
        }
        result
    }
}

impl StateManager {
    pub fn new(num_streams: usize) -> Self {
        // Create streams with unique IDs (0, 1, 2, ...)
        let streams = (0..num_streams).map(Stream::new).collect();
        StateManager { streams }
    }

    pub fn global_state(&self) -> String {
        use sha2::{Digest, Sha256};

        // Collect Merkle roots from all streams
        let merkle_roots: Vec<String> = self
            .streams
            .iter()
            .map(|stream| stream.generate_merkle_root())
            .collect();

        // Combine Merkle roots into a single string
        let combined_roots = merkle_roots.join("");

        // Hash the combined string to produce the global state hash
        let mut hasher = Sha256::new();
        hasher.update(combined_roots.as_bytes());
        let result = hasher.finalize();
        hex::encode(result)
    }

    pub fn add_transaction_to_stream(&mut self, mut transaction: Transaction) {
        // 1. Get the number of streams we have
        let num_streams = self.streams.len();
    
        // 2. Hash the transaction to get a unique number
        let transaction_hash = Self::calculate_transaction_hash(&transaction);
    
        // 3. Decide the stream using hash % number_of_streams
        let stream_id = transaction_hash % num_streams;
    
        // Apply the VDF to the transaction's timestamp
        transaction.verified_timestamp = self.apply_vdf(transaction.timestamp);
    
        // Debug output to track assignment
        println!(
            "Transaction: {:?} assigned to Stream {} (Hash: {}, Verified Timestamp: {}, Total Streams: {})",
            transaction, stream_id, transaction_hash, transaction.verified_timestamp, num_streams
        );
    
        // 4. Add the transaction to the chosen stream
        self.streams[stream_id].add_transaction(transaction);
    }
    

    fn calculate_transaction_hash(transaction: &Transaction) -> usize {
        use sha2::{Sha256, Digest};

        // Combine transaction data into a single string
        let input = format!(
            "{}{}{}{}",
            transaction.sender, transaction.receiver, transaction.amount, transaction.timestamp
        );

        // Hash the string
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();

        // Convert the first few bytes of the hash to a number
        let hash_bytes = &result[..8];
        usize::from_be_bytes(hash_bytes.try_into().unwrap())
    }
}
