use crate::core::block::Block;
use crate::core::transaction::{Transaction, TransactionPool};
use std::collections::HashMap;
use crate::smart_contracts::{SmartContract, VirtualMachine};
use rand::Rng;
use rayon::prelude::*;

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
    pub transaction_pool: TransactionPool,
    pub balances: HashMap<String, u64>,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut blockchain = Blockchain {
            chain: vec![],
            difficulty: 2,
            transaction_pool: TransactionPool::new(),
            balances: HashMap::new(),
        };
        blockchain.chain.push(Block::new(0, 0, "[]".to_string(), "0".to_string()));
        blockchain
    }

    pub fn add_block(&mut self, use_pow: bool) {
        if use_pow {
            self.add_block_with_pow();
        } else {
            self.add_block_with_pos();
        }
    }

    pub fn add_block_with_pow(&mut self) {
        if self.chain.is_empty() {
            println!("Blockchain is empty. Cannot add a block.");
            return;
        }

        let previous_block = self.chain.last().expect("Expected a previous block");
        let transactions = self.validate_transactions();
        let data = serde_json::to_string(&transactions).expect("Failed to serialize transactions");
        let mut new_block = Block::new(self.chain.len() as u64, 0, data, previous_block.hash.clone());

        for nonce in 0..10_000_000 {
            new_block.nonce = nonce;
            let hash = new_block.calculate_hash();
            if hash.starts_with(&"0".repeat(self.difficulty)) {
                new_block.hash = hash;
                break;
            }
        }

        self.chain.push(new_block);
        self.transaction_pool.clear();
        self.update_balances();
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        if self.validate_transaction(&transaction) {
            println!("Adding transaction: {:?}", transaction);
            self.transaction_pool.add_transaction(transaction);
        } else {
            println!("Transaction validation failed: {:?}", transaction);
        }
    }

    pub fn validate_transaction(&self, transaction: &Transaction) -> bool {
        let sender_balance = self.balances.get(&transaction.sender).cloned().unwrap_or(0);
        let is_valid = sender_balance >= transaction.amount + transaction.fee;
        println!("Validating transaction: {:?}, Sender balance: {}, Is valid: {}", transaction, sender_balance, is_valid);
        is_valid
    }

    pub fn update_balances(&mut self) {
        for block in &self.chain {
            let transactions: Vec<Transaction> = match serde_json::from_str(&block.data) {
                Ok(transactions) => transactions,
                Err(e) => {
                    eprintln!("Failed to deserialize transactions: {:?}", e);
                    continue;
                }
            };
            for transaction in transactions {
                let sender_balance = self.balances.get(&transaction.sender).cloned().unwrap_or(0);
                let receiver_balance = self.balances.get(&transaction.receiver).cloned().unwrap_or(0);
                self.balances.insert(transaction.sender.clone(), sender_balance - transaction.amount - transaction.fee);
                self.balances.insert(transaction.receiver.clone(), receiver_balance + transaction.amount);
                self.balances.insert(transaction.receiver.clone(), receiver_balance + transaction.fee);
            }
        }
    }

    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            println!("Checking block {}: current hash = {}, calculated hash = {}", i, current_block.hash, current_block.calculate_hash());
            println!("Previous hash = {}, expected previous hash = {}", current_block.previous_hash, previous_block.hash);

            if current_block.hash != current_block.calculate_hash() {
                return false;
            }

            if current_block.previous_hash != previous_block.hash {
                return false;
            }
        }
        true
    }

    pub fn execute_contract(&mut self, contract: &mut SmartContract, function_name: &str, params: &[i32]) -> Result<i32, Box<dyn std::error::Error>> {
        let mut vm = VirtualMachine::new();
        vm.execute(&contract.code, params)?;
        contract.execute(function_name, params)
    }

    pub fn resolve_fork(&mut self, other_chain: Vec<Block>) {
        if other_chain.len() > self.chain.len() && self.is_chain_valid() {
            self.chain = other_chain;
            self.update_balances();
        }
    }

    pub fn select_validator(&self) -> String {
        self.balances.iter().max_by_key(|entry| entry.1).map(|(k, _)| k.clone()).unwrap_or_default()
    }

    pub fn reward_validator(&mut self, validator: &str, reward: u64) {
        let balance = self.balances.entry(validator.to_string()).or_insert(0);
        *balance += reward;
    }

    pub fn add_block_with_pos(&mut self) {
        if self.chain.is_empty() {
            println!("Blockchain is empty. Cannot add a block.");
            return;
        }

        let previous_block = self.chain.last().expect("Expected a previous block");
        let transactions = self.validate_transactions();
        let data = serde_json::to_string(&transactions).expect("Failed to serialize transactions");
        let mut new_block = Block::new(self.chain.len() as u64, 0, data, previous_block.hash.clone());

        let validator = self.select_validator();
        println!("Selected validator: {}", validator);

        new_block.hash = new_block.calculate_hash();
        self.chain.push(new_block);
        self.transaction_pool.clear();
        self.update_balances();
    }

    pub fn adjust_difficulty(&mut self) {
        let target_time = 10; // Target time in seconds for block creation
        let actual_time = self.calculate_actual_time(); // Implement this method to calculate the time taken for recent blocks

        if actual_time < target_time {
            self.difficulty += 1;
        } else if actual_time > target_time {
            self.difficulty -= 1;
        }
    }

    fn calculate_actual_time(&self) -> u64 {
        // Implement logic to calculate the actual time taken for recent blocks
        0 // Placeholder
    }

    pub fn slash_validator(&mut self, validator: &str, penalty: u64) {
        if let Some(balance) = self.balances.get_mut(validator) {
            *balance = balance.saturating_sub(penalty);
        }
    }

    pub fn prioritize_transactions(&mut self) {
        let mut transactions: Vec<_> = self.transaction_pool.transactions.iter().cloned().collect();
        transactions.sort_by(|a, b| b.fee.cmp(&a.fee));
        self.transaction_pool.transactions = transactions.into();
    }

    pub fn deploy_contract(&mut self, code: String) -> SmartContract {
        SmartContract::new(code)
    }

    pub fn call_contract(&mut self, _caller: &mut SmartContract, callee: &mut SmartContract, function_name: &str, params: &[i32]) -> Result<i32, Box<dyn std::error::Error>> {
        callee.execute(function_name, params)
    }

    pub fn call_contract_with_return(&mut self, caller: &mut SmartContract, callee: &mut SmartContract, function_name: &str, params: &[i32]) -> Result<i32, Box<dyn std::error::Error>> {
        // Logic to call another contract and handle return values
        callee.execute(function_name, params)
    }

    pub fn validate_transactions(&self) -> Vec<Transaction> {
        self.transaction_pool.transactions.iter()
            .filter(|tx| self.validate_transaction(tx))
            .cloned()
            .collect()
    }

    pub fn mine_block_optimized(&mut self, difficulty: usize) {
        let previous_block = self.chain.last().unwrap();
        let transactions = self.validate_transactions_parallel();
        let data = serde_json::to_string(&transactions).unwrap();
        let mut new_block = Block::new(self.chain.len() as u64, 0, data, previous_block.hash.clone());

        let hash = (0..)
            .take(1_000_000) // Limit the range for demonstration purposes
            .collect::<Vec<_>>() // Convert to Vec for parallel iteration
            .into_par_iter()
            .map(|nonce| {
                let mut block = new_block.clone(); // Clone the block for each iteration
                block.nonce = nonce;
                block.calculate_hash()
            })
            .find_any(|hash| hash.starts_with(&"0".repeat(difficulty)))
            .unwrap();

        new_block.hash = hash;
        self.chain.push(new_block);
        self.transaction_pool.clear();
        self.update_balances();
    }

    pub fn process_transactions_in_batches(&mut self, batch_size: usize) {
        let transactions: Vec<Transaction> = self.transaction_pool.transactions.drain(..batch_size).collect();
        for transaction in transactions {
            if self.validate_transaction(&transaction) {
                self.apply_transaction(&transaction);
            }
        }
    }

    pub fn apply_transaction(&mut self, transaction: &Transaction) {
        let sender_balance = self.balances.get_mut(&transaction.sender).unwrap();
        *sender_balance -= transaction.amount + transaction.fee;
        let receiver_balance = self.balances.entry(transaction.receiver.clone()).or_insert(0);
        *receiver_balance += transaction.amount;
    }

    pub fn validate_transactions_parallel(&self) -> Vec<Transaction> {
        self.transaction_pool.transactions.par_iter()
            .filter(|tx| self.validate_transaction(tx))
            .cloned()
            .collect()
    }

    pub fn validate_transaction_security(&self, transaction: &Transaction) -> bool {
        // Add additional security checks
        self.validate_transaction(transaction) && transaction.verify_signatures(&[&transaction.sender.as_bytes()])
    }
} 