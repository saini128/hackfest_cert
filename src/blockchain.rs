use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use chrono::Utc;
use std::fmt;

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
}

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct Block {
    pub timestamp: u128,
    pub transaction: Transaction,
    pub previous_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(timestamp: u128, transaction: Transaction, previous_hash: String) -> Self {
        let mut block = Block {
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
        let data = format!("{:?}{:?}{:?}",self.timestamp, self.transaction, self.previous_hash);
        hasher.update(data.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

#[derive(Serialize, Deserialize, Debug,Clone)]
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
        let new_block = Block::new(timestamp, transaction, previous_hash);
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

impl fmt::Display for Blockchain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut i = 0;
        for block in &self.blocks {
            writeln!(f, "Block {}:", i)?;
            writeln!(f, "  Timestamp: {}", block.timestamp)?;
            writeln!(f, "  Transaction: {:?}", block.transaction)?;
            writeln!(f, "  Previous Hash: {}", block.previous_hash)?;
            writeln!(f, "  Hash: {}", block.hash)?;
            writeln!(f)?;
            i += 1;
        }
        Ok(())
    }
}
