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
    // Create a new Tokio runtime
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
    let network = Network::new();

    // Initialize metrics
    let registry = Arc::new(Registry::new());
    let metrics = Metrics::new();
    metrics.register(&registry);

    // Start the network server
    rt.spawn(async move {
        println!("Starting network server...");
        if let Err(e) = network.start_server().await {
            eprintln!("Network server error: {:?}", e);
        }
    });

    // Start the API server
    start_api();

    // Start the metrics server
    rt.spawn(async move {
        println!("Starting metrics server...");
        serve_metrics(registry.clone()).await;
    });

    // Block the main thread until the runtime is shut down
    rt.block_on(async {
        // You can add additional async tasks here if needed
        println!("All services are running.");
    });

    let storage = Arc::new(Storage::new("contract_state"));

    let deploy_contract = warp::path("deploy")
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::any().map(move || Arc::clone(&storage)))
        .map(|code: String, storage: Arc<Storage>| {
            let contract = SmartContract::new(code);
            contract.save_state(&storage, "contract_id");
            warp::reply::json(&"Contract deployed")
        });

    let routes = deploy_contract;
    warp::serve(routes).run(([127, 0, 0, 1], 3030));
} 