use secp256k1::{
    rand::{self, rngs, SeedableRng},
    PublicKey, SecretKey,
};
use sha2::{Digest, Sha256};
use hex::encode;

fn generate_keypair() -> (SecretKey, PublicKey) {
    let secp = secp256k1::Secp256k1::new();
    let mut rng = rngs::StdRng::seed_from_u64(rand::random::<u64>());
    secp.generate_keypair(&mut rng)
}

fn generate_hash(input: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    encode(result)
}

pub fn new_hash() -> String {
    let (_secret_key, public_key) = generate_keypair();
    let mut hash = generate_hash(&public_key.serialize());
    while hash.len() != 64 {
        let (_secret_key, public_key) = generate_keypair();
        hash = generate_hash(&public_key.serialize());
    }
    hash
}