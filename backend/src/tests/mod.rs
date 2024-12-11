#[cfg(test)]
mod tests {
    use crate::core::block::Block;
    use crate::core::blockchain::Blockchain;
    use crate::core::transaction::{Transaction, TransactionPool};
    use ring::signature::{Ed25519KeyPair, KeyPair};
    use ring::rand::SystemRandom;
    use crate::smart_contracts::SmartContract;
    use blockchain_project::storage::Storage;

    #[test]
    fn test_block_creation() {
        let block = Block::new(0, 0, "Test Data".to_string(), "0".to_string());
        assert_eq!(block.index, 0);
        assert_eq!(block.data, "Test Data");
    }

    #[test]
    fn test_blockchain_validity() {
        let mut blockchain = Blockchain::new();
        blockchain.balances.insert("Alice".to_string(), 100);

        blockchain.add_transaction(Transaction {
            sender: "Alice".to_string(),
            receiver: "Bob".to_string(),
            amount: 50,
            fee: 1,
            signatures: Vec::new(),
        });

        blockchain.add_block(true);
        assert!(blockchain.is_chain_valid());
    }

    #[test]
    fn test_transaction_pool() {
        let mut pool = TransactionPool::new();
        let transaction = Transaction {
            sender: "Alice".to_string(),
            receiver: "Bob".to_string(),
            amount: 50,
            fee: 1,
            signatures: Vec::new(),
        };
        pool.add_transaction(transaction.clone());
        assert_eq!(pool.get_transactions().len(), 1);
        assert_eq!(pool.get_transactions()[0], transaction);
    }

    #[test]
    fn test_block_mining() {
        let mut block = Block::new(1, 0, "Test Data".to_string(), "0".to_string());
        block.mine_block(2);
        assert!(block.hash.starts_with("00"));
    }

    #[test]
    fn test_invalid_chain() {
        let mut blockchain = Blockchain::new();
        blockchain.balances.insert("Alice".to_string(), 100);

        blockchain.add_transaction(Transaction {
            sender: "Alice".to_string(),
            receiver: "Bob".to_string(),
            amount: 50,
            fee: 1,
            signatures: Vec::new(),
        });

        blockchain.add_block(true);

        // Tamper with the blockchain
        blockchain.chain[1].data = "Tampered Data".to_string();
        assert!(!blockchain.is_chain_valid());
    }

    #[test]
    fn test_multiple_transactions() {
        let mut blockchain = Blockchain::new();
        
        // Initialize balances for the senders
        blockchain.balances.insert("Alice".to_string(), 100);
        blockchain.balances.insert("Charlie".to_string(), 100);

        blockchain.add_transaction(Transaction {
            sender: "Alice".to_string(),
            receiver: "Bob".to_string(),
            amount: 50,
            fee: 1,
            signatures: Vec::new(),
        });
        blockchain.add_transaction(Transaction {
            sender: "Charlie".to_string(),
            receiver: "Dave".to_string(),
            amount: 30,
            fee: 1,
            signatures: Vec::new(),
        });
        blockchain.add_block(true);

        let transactions: Vec<Transaction> = serde_json::from_str(&blockchain.chain[1].data).unwrap();
        assert_eq!(transactions.len(), 2);
    }

    #[test]
    fn test_transaction_signature() {
        let rng = SystemRandom::new();
        let keypair = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
        let keypair = Ed25519KeyPair::from_pkcs8(keypair.as_ref()).unwrap();

        let mut transaction = Transaction::new("Alice".to_string(), "Bob".to_string(), 50, 1);
        transaction.sign(&keypair);

        assert!(transaction.verify(keypair.public_key().as_ref()));
    }

    #[test]
    fn test_balance_check() {
        let mut blockchain = Blockchain::new();
        blockchain.balances.insert("Alice".to_string(), 100);

        let transaction = Transaction {
            sender: "Alice".to_string(),
            receiver: "Bob".to_string(),
            amount: 50,
            fee: 1,
            signatures: Vec::new(),
        };

        assert!(blockchain.validate_transaction(&transaction));
    }

    #[test]
    fn test_smart_contract_execution() {
        let mut contract = SmartContract::new("add".to_string());
        let result = contract.execute("add", &[1, 2, 3]);
        assert_eq!(result.unwrap(), 6);
    }

    #[test]
    fn test_contract_state_persistence() {
        let storage = Storage::new("test_state");
        let mut contract = SmartContract::new("add".to_string());
        contract.state.insert("key".to_string(), 42);
        contract.save_state(&storage, "test_contract");

        let mut loaded_contract = SmartContract::new("add".to_string());
        loaded_contract.load_state(&storage, "test_contract");
        assert_eq!(loaded_contract.state.get("key"), Some(&42));
    }

    #[test]
    fn test_role_based_access_control() {
        let mut contract = SmartContract::new("add".to_string());
        contract.add_role("admin".to_string());

        // Test with authorized role
        let result = contract.execute_with_role("admin", "add", &[1, 2, 3]);
        assert_eq!(result.unwrap(), 6);

        // Test with unauthorized role
        let result = contract.execute_with_role("user", "add", &[1, 2, 3]);
        assert!(result.is_err());
    }

    #[test]
    fn test_contract_upgradability() {
        let mut contract = SmartContract::new("add".to_string());
        contract.state.insert("key".to_string(), 42);

        // Upgrade contract
        contract.upgrade("multiply".to_string());

        // Ensure state is preserved
        assert_eq!(contract.state.get("key"), Some(&42));

        // Test new functionality
        let result = contract.execute("multiply", &[2, 3]);
        assert_eq!(result.unwrap(), 6);
    }

    #[test]
    fn test_event_emission() {
        let contract = SmartContract::new("add".to_string());
        contract.emit_event("TestEvent", "EventData");

        // You might need to capture stdout or use a mock to verify the event emission
    }

    #[test]
    fn test_error_handling() {
        let mut contract = SmartContract::new("add".to_string());

        // Test with valid function
        let result = contract.execute_with_error_handling("add", &[1, 2, 3]);
        assert_eq!(result.unwrap(), 6);

        // Test with invalid function
        let result = contract.execute_with_error_handling("invalid", &[1, 2, 3]);
        assert!(result.is_err());
    }
} 