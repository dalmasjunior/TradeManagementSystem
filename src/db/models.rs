//! This module contains the definitions of various data models used in the application.
//!
//! It includes submodules for `User`, `Trade`, and `Wallet` data models, each of which is
//! responsible for representing and interacting with corresponding data in the application.
//! Additionally, the module includes submodules for testing each data model.
//!
//! # Submodules
//!
//! - [`user`](user/index.html): Contains the `User` data model and related methods.
//! - [`trade`](trade/index.html): Contains the `Trade` data model and related methods.
//! - [`wallet`](wallet/index.html): Contains the `Wallet` data model and related methods.
//! - [`user_test`](user_test/index.html): Contains unit tests for the `User` data model.
//! - [`trade_test`](trade_test/index.html): Contains unit tests for the `Trade` data model.
//! - [`wallet_test`](wallet_test/index.html): Contains unit tests for the `Wallet` data model.
//!
//! # Examples
//!
//! ```rust
//! // Import data models
//! mod models;
//!
//! // Import user, trade, and wallet data models
//! use models::{user::User, trade::Trade, wallet::Wallet};
//!
//! // List all users, trades, and wallets
//! let users = User::list(&mut connection);
//! let trades = Trade::list(&mut connection);
//! let wallets = Wallet::list(&mut connection);
//! ```
//!
//! # Note
//! This module assumes the availability of a database connection (`SqliteConnection` in this case) for data retrieval and manipulation.
//!

// Import user data model
pub mod user;

// Import trade data model
pub mod trade;

// Import wallet data model
pub mod wallet;

// Import trade tests (only included in test builds)
#[cfg(test)]
mod trade_test;
