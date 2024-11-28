use crate::ledger::streams::Stream;

pub struct StateManager {
    pub streams: Vec<Stream>,
}

impl StateManager {
    pub fn new(num_streams: usize) -> Self {
        let streams = (0..num_streams).map(Stream::new).collect();
        StateManager { streams }
    }

    pub fn aggregate_state(&self) -> String {
        // Aggregate Merkle roots from all streams
        self.streams
            .iter()
            .map(|stream| stream.generate_merkle_root())
            .collect::<Vec<_>>()
            .join("_")
    }
}
