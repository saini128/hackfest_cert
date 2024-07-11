mod blockchain;
mod storage;

use blockchain::{Blockchain, Transaction};
use storage::Storage;

fn main() {
    let storage = Storage::new("blockchain.db");

    let mut blockchain = if let Some(loaded_blockchain) = storage.load_blockchain() {
        loaded_blockchain
    } else {
        Blockchain::new()
    };

    let transaction = Transaction {
        sender: "Kanishk".to_string(),
        receiver: "Akarsh".to_string(),
        amount: 150.50,
    };

    blockchain.add_block(transaction);

    println!("Is blockchain valid? {}", blockchain.is_valid());
    
    // Print the blockchain
    println!("Blockchain:\n{}", blockchain);

    storage.store_blockchain(&blockchain);
}
