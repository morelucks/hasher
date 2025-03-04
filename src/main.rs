use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize)]
struct Block {
    index: u32,
    timestamp: i64,
    prev_hash: String,
    data: String,
    hash: String,
}

impl Block {
    fn new(index: u32, prev_hash: String, data: String) -> Self {
        let timestamp = Utc::now().timestamp();
        let mut block = Block {
            index,
            timestamp,
            prev_hash,
            data,
            hash: String::new(),
        };
        block.hash = block.calculate_hash();
        block
    }

    fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}{}", self.index, self.timestamp, self.data));
        format!("{:x}", hasher.finalize())
    }
}

fn main() {
    println!("🚀 Blockchain Initialized!");
}
