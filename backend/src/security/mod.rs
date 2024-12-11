use ring::signature::{Ed25519KeyPair, Signature, KeyPair, ED25519};

pub fn generate_keypair() -> Ed25519KeyPair {
    let rng = ring::rand::SystemRandom::new();
    Ed25519KeyPair::generate_pkcs8(&rng).unwrap()
}

pub fn sign_data(keypair: &Ed25519KeyPair, data: &[u8]) -> Signature {
    keypair.sign(data)
} 