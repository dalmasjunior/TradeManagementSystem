//! This module defines a `Wallet` struct and associated methods for managing wallet information.
//!
//! The `Wallet` struct represents a wallet in the application, with attributes such as wallet ID, hash, balance,
//! and timestamps for creation and update.
//! 
//! The module provides methods for retrieving wallet data from the database, creating new wallets, and updating wallet balances.
//! Additionally, it includes utility methods for generating a new wallet hash and creating a new wallet struct.
//! 
//! # Examples
//! 
//! ```rust
//! use crate::models::wallet::Wallet;
//!
//! // List all wallets in the database
//! let wallets = Wallet::list(&mut connection);
//!
//! // Find a wallet by ID
//! if let Some(wallet) = Wallet::find_by_id(&mut connection, "wallet_id".to_string()) {
//!     println!("Found wallet: {:?}", wallet);
//! }
//!
//! // Find a wallet by hash
//! if let Some(wallet) = Wallet::find_by_hash(&mut connection, "wallet_hash".to_string()) {
//!     println!("Found wallet: {:?}", wallet);
//! }
//!
//! // Create a new wallet
//! if let Some(new_wallet) = Wallet::create(&mut connection) {
//!     println!("Created new wallet: {:?}", new_wallet);
//! }
//!
//! // Update wallet balance
//! if let Some(updated_wallet) = Wallet::update_balance(&mut connection, "wallet_id".to_string(), 100.0) {
//!     println!("Updated wallet balance: {:?}", updated_wallet);
//! }
//! ```
//!
//! # Note
//! This module assumes the availability of a database connection (`SqliteConnection` in this case) for wallet data retrieval and manipulation.

use uuid::Uuid;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;

use super::super::schema::wallet;
use super::super::schema::wallet::dsl::{
    id as id_dsl,
    wallet as wallet_dsl, 
    balance as balance_dsl,
    hash as hash_dsl,
};

use crate::utils::hash::new_hash;

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[diesel(table_name = crate::db::schema::wallet)]
pub struct Wallet {
    pub id: String,
    pub hash: String,
    pub balance: f32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl Wallet {
    pub fn list(conn: &mut SqliteConnection) -> Vec<Self> {
        wallet_dsl
            .order(wallet::id.desc())
            .load::<Wallet>(conn)
            .expect("Error loading wallets")
    }
    
    pub fn find_by_id(conn: &mut SqliteConnection, id: String) -> Option<Self> {
        
        let wallet = wallet_dsl
            .filter(id_dsl.eq(id.clone()))
            .first::<Wallet>(conn)
            .optional()
            .expect("Error loading wallet");
        
        match wallet {
            Some(wallet) => Some(wallet),
            None => None,
        }
    }

    pub fn find_by_hash(conn: &mut SqliteConnection, hash: String) -> Option<Self> {
        let wallet = wallet_dsl
            .filter(hash_dsl.eq(hash))
            .first::<Wallet>(conn)
            .optional()
            .expect("Error loading wallet");

        match wallet {
            Some(wallet) => Some(wallet),
            None => None,
        }
    }

    pub fn create(conn: &mut SqliteConnection) -> Option<Self> {
        let new_id = Uuid::new_v4().as_hyphenated().to_string();
        let new_hash = new_hash();
        let new_wallet = Self::new_wallet_struct(new_id, new_hash.clone(), 0.0);

        diesel::insert_into(wallet_dsl)
            .values(&new_wallet)
            .execute(conn)
            .expect("Error saving new wallet");
        
        Self::find_by_hash(conn, new_hash)
    }

    fn new_wallet_struct(id: String, hash: String, balance: f32) -> Self {
        Self {
            id: id,
            hash: hash,
            balance: balance,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }

    pub fn update_balance(conn: &mut SqliteConnection, id: String, balance: f32) -> Option<Self> {
        if let Some(mut _wallet) = Self::find_by_id(conn, id.clone()) {
            diesel::update(wallet_dsl.find(id.clone()))
                .set(balance_dsl.eq(balance))
                .execute(conn)
                .expect("Error updating wallet");
            Self::find_by_id(conn, id)
        } else {
            None
        }
    }
}


