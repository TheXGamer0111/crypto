use serde::{Serialize, Deserialize};
use ring::signature::{Ed25519KeyPair, Signature, UnparsedPublicKey, ED25519};
use std::fmt;
use std::collections::VecDeque;

#[derive(Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub fee: u64,
    pub nonce: u64,
    #[serde(skip)]
    pub signatures: Vec<Signature>,
    pub required_signatures: usize,
}

impl Transaction {
    pub fn new(sender: String, receiver: String, amount: u64, fee: u64, required_signatures: usize) -> Self {
        Transaction {
            sender,
            receiver,
            amount,
            fee,
            nonce: 0,
            signatures: Vec::new(),
            required_signatures,
        }
    }

    pub fn sign(&mut self, keypair: &Ed25519KeyPair) {
        let message = format!("{}{}{}{}", self.sender, self.receiver, self.amount, self.fee);
        self.signatures.push(keypair.sign(message.as_bytes()));
    }

    pub fn verify(&self, public_key: &[u8]) -> bool {
        for signature in &self.signatures {
            let message = format!("{}{}{}{}", self.sender, self.receiver, self.amount, self.fee);
            let public_key = UnparsedPublicKey::new(&ED25519, public_key);
            if !public_key.verify(message.as_bytes(), signature.as_ref()).is_ok() {
                return false;
            }
        }
        true
    }

    pub fn add_signature(&mut self, signature: Signature) {
        self.signatures.push(signature);
    }

    pub fn verify_signatures(&self, public_keys: &[&[u8]]) -> bool {
        if self.signatures.len() != public_keys.len() {
            return false;
        }

        for (i, signature) in self.signatures.iter().enumerate() {
            let message = format!("{}{}{}{}", self.sender, self.receiver, self.amount, self.fee);
            let public_key = UnparsedPublicKey::new(&ED25519, public_keys[i]);
            if !public_key.verify(message.as_bytes(), signature.as_ref()).is_ok() {
                return false;
            }
        }
        true
    }

    pub fn verify_multi_signature(&self, public_keys: &[&[u8]]) -> bool {
        // Implement logic to verify multiple signatures
        self.verify_signatures(public_keys)
    }

    pub fn is_fully_signed(&self) -> bool {
        self.signatures.len() >= self.required_signatures
    }
}

// Implement PartialEq manually, excluding the signature field
impl PartialEq for Transaction {
    fn eq(&self, other: &Self) -> bool {
        self.sender == other.sender &&
        self.receiver == other.receiver &&
        self.amount == other.amount
    }
}

// Implement Debug manually, excluding the signature field
impl fmt::Debug for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Transaction")
            .field("sender", &self.sender)
            .field("receiver", &self.receiver)
            .field("amount", &self.amount)
            .finish()
    }
}

pub struct TransactionPool {
    pub transactions: VecDeque<Transaction>,
}

impl TransactionPool {
    pub fn new() -> Self {
        TransactionPool {
            transactions: VecDeque::new(),
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        if transaction.amount > 0 {
            self.transactions.push_back(transaction);
        }
    }

    pub fn get_transactions(&self) -> Vec<Transaction> {
        self.transactions.iter().cloned().collect()
    }

    pub fn clear(&mut self) {
        self.transactions.clear();
    }
} 