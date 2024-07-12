use sled::{Db};
use crate::blockchain::{Blockchain};
use serde_json;

pub struct Storage {
    db: Db,
}

impl Storage {
    pub fn new(path: &str) -> Self {
        let db = sled::open(path).unwrap();
        Storage { db }
    }

    // pub fn store_blockchain(&self, blockchain: &Blockchain) {
    //     let serialized = serde_json::to_string(blockchain).unwrap();
    //     self.db.insert("blockchain", serialized.as_bytes()).unwrap();
    //     self.db.flush().unwrap();
    // }

    pub fn load_blockchain(&self) -> Option<Blockchain> {
        match self.db.get("blockchain").unwrap() {
            Some(data) => {
                let serialized = String::from_utf8(data.to_vec()).unwrap();
                Some(serde_json::from_str(&serialized).unwrap())
            },
            None => None,
        }
    }
}
