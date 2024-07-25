use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use actix_web::{web, App, HttpServer, Responder, HttpResponse, http::header};
use serde_json::json;
use actix_cors::Cors;

mod blockchain;
mod storage;
use crate::blockchain::{Blockchain, Transaction}; 
use crate::storage::Storage; 

pub type SharedBlockchain = Arc<Mutex<Blockchain>>;
pub type SharedStorage = Arc<Storage>;

#[derive(Debug, Deserialize)]
struct TransactionRequest {
    sender: String,
    receiver: String,
    amount: f64,
}

#[derive(Debug, Serialize)]
struct BlockResponse {
    timestamp: u128,
    transaction: Transaction,
    previous_hash: String,
    hash: String,
}

async fn add_transaction(
    transaction: web::Json<TransactionRequest>,
    blockchain: web::Data<SharedBlockchain>,
    storage: web::Data<SharedStorage>,
) -> impl Responder {
    let mut blockchain = blockchain.lock().unwrap();
    let transaction = Transaction {
        sender: transaction.sender.clone(),
        receiver: transaction.receiver.clone(),
        amount: transaction.amount,
    };
    blockchain.add_block(transaction.clone());
    storage.store_blockchain(&blockchain);

    let response = json!({
        "message": "Transaction added successfully",
        "transaction": transaction,
    });

    HttpResponse::Ok().json(response)
}

async fn get_last_10_blocks(blockchain: web::Data<SharedBlockchain>) -> impl Responder {
    let blockchain = blockchain.lock().unwrap();
    let last_10_blocks: Vec<BlockResponse> = blockchain.blocks.iter().rev().take(10).map(|block| BlockResponse {
        timestamp: block.timestamp,
        transaction: block.transaction.clone(),
        previous_hash: block.previous_hash.clone(),
        hash: block.hash.clone(),
    }).collect();

    HttpResponse::Ok().json(last_10_blocks)
}

async fn get_all_blocks(blockchain: web::Data<SharedBlockchain>) -> impl Responder {
    let blockchain = blockchain.lock().unwrap();
    let all_blocks: Vec<BlockResponse> = blockchain.blocks.iter().map(|block| BlockResponse {
        timestamp: block.timestamp,
        transaction: block.transaction.clone(),
        previous_hash: block.previous_hash.clone(),
        hash: block.hash.clone(),
    }).collect();

    HttpResponse::Ok().json(all_blocks)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let storage = Arc::new(Storage::new("blockchain.db"));
    let blockchain = if let Some(loaded_blockchain) = storage.load_blockchain() {
        Arc::new(Mutex::new(loaded_blockchain))
    } else {
        Arc::new(Mutex::new(Blockchain::new()))
    };

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::default()
                .allow_any_origin()
                .allowed_methods(vec!["GET", "POST"])
                .allowed_headers(vec![
                    header::ACCEPT,
                    header::CONTENT_TYPE,
                ])
            )
            .app_data(web::Data::new(blockchain.clone()))
            .app_data(web::Data::new(storage.clone()))
            .route("/add_transaction", web::post().to(add_transaction))
            .route("/last_10_blocks", web::get().to(get_last_10_blocks))
            .route("/all_blocks", web::get().to(get_all_blocks))
    })    
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
