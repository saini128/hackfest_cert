use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use chrono::Utc;

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub transaction: Transaction,
    pub previous_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, timestamp: u128, transaction: Transaction, previous_hash: String) -> Self {
        let mut block = Block {
            index,
            timestamp,
            transaction,
            previous_hash: previous_hash.clone(),
            hash: String::new(),
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        let data = format!("{:?}{:?}{:?}{:?}", self.index, self.timestamp, self.transaction, self.previous_hash);
        hasher.update(data.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain { blocks: vec![] }
    }

    pub fn add_block(&mut self, transaction: Transaction) {
        let index = self.blocks.len() as u64;
        let timestamp = Utc::now().timestamp_millis() as u128;
        let previous_hash = if index == 0 {
            String::from("0")
        } else {
            self.blocks.last().unwrap().hash.clone()
        };
        let new_block = Block::new(index, timestamp, transaction, previous_hash);
        self.blocks.push(new_block);
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.blocks.len() {
            let current = &self.blocks[i];
            let previous = &self.blocks[i - 1];

            if current.hash != current.calculate_hash() {
                return false;
            }

            if current.previous_hash != previous.hash {
                return false;
            }
        }
        true
    }
}
