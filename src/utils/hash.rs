//! This module provides functions for working with cryptographic key generation and hashing using the `secp256k1`, `sha2`, and `hex` crates.
//!
//! The provided functions include:
//!
//! - `generate_keypair`: Generates a new pair of secret and public keys using the `secp256k1` elliptic curve algorithm.
//! - `generate_hash`: Generates a SHA-256 hash from the provided input data.
//! - `new_hash`: Generates a new SHA-256 hash using a randomly generated public key.
//!
//! # Examples
//!
//! ```
//! use secp256k1::{
//!     rand::{self, rngs, SeedableRng},
//!     PublicKey, SecretKey,
//! };
//! use sha2::{Digest, Sha256};
//! use hex::encode;
//!
//! /// Generates a new pair of secret and public keys.
//! fn generate_keypair() -> (SecretKey, PublicKey) {
//!     // ... implementation details ...
//! }
//!
//! /// Generates a SHA-256 hash from the provided input data.
//! fn generate_hash(input: &[u8]) -> String {
//!     // ... implementation details ...
//! }
//!
//! /// Generates a new SHA-256 hash using a randomly generated public key.
//! pub fn new_hash() -> String {
//!     // ... implementation details ...
//! }
//!
//! // Example usage
//! let hash = new_hash();
//! println!("Generated Hash: {}", hash);
//! ```

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