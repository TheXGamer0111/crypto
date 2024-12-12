mod api;
mod core;
mod monitoring;
mod network;
mod smart_contracts;


#[cfg(test)]
mod tests;

use std::sync::Arc;
use tokio::runtime::Runtime;
use prometheus::Registry;
use crate::network::Network;
use crate::api::start_api;
use crate::monitoring::{Metrics, serve_metrics};
use crate::core::blockchain::Blockchain;
use crate::core::transaction::Transaction;
use warp::Filter;
use crate::smart_contracts::SmartContract;
use blockchain_project::storage::Storage;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let rt = Runtime::new().unwrap();

    println!("Starting the blockchain project...");

    // Initialize the blockchain
    let mut blockchain = Blockchain::new();

    // Initialize Alice's balance
    blockchain.balances.insert("Alice".to_string(), 100);

    // Add a sample transaction with a fee
    blockchain.add_transaction(Transaction {
        sender: "Alice".to_string(),
        receiver: "Bob".to_string(),
        amount: 50,
        fee: 1,
        signatures: Vec::new(),
    });

    // Choose consensus mechanism
    let use_pow = true; // Set to false to use PoS

    if use_pow {
        blockchain.add_block_with_pow();
    } else {
        blockchain.add_block_with_pos();
    }

    // Validate the blockchain
    println!("Is blockchain valid? {}", blockchain.is_chain_valid());

    // Initialize the network
    let network = Arc::new(Network::new());

    // Start the P2P server
    let network_clone = Arc::clone(&network);
    rt.spawn(async move {
        if let Err(e) = network_clone.start_server().await {
            eprintln!("P2P server error: {:?}", e);
        }
    });

    // Start the API server
    rt.block_on(async {
        start_api(network.clone()).await;
    });

    // Block the main thread until the runtime is shut down
    rt.block_on(async {
        println!("All services are running.");
    });
} 