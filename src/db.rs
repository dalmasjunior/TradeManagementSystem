//! This module provides database connectivity and setup for Diesel ORM and migrations.
//!
//! It includes functions to establish a database connection, handle connection pooling, and perform migrations.
//! The `establish_connection` function initializes the database connection pool, allowing efficient connection
//! reuse across multiple threads.
//!
//! The module also contains constants for embedded migrations, allowing seamless migration execution.
//!
//! # Examples
//!
//! ```rust
//! use crate::db::{DbPool, establish_connection};
//!
//! // ... imports ...
//!
//! // Initialize the database connection pool.
//! let pool: DbPool = establish_connection();
//!
//! // ... other database operations ...
//! ```
//!
//! # Note
//! Make sure to configure your environment variables (e.g., `DATABASE_URL`) to ensure proper database connection setup and migration execution.

use std::env;
use diesel_migrations::EmbeddedMigrations;
use dotenv::dotenv;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;

pub mod models;
pub mod schema;

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;
// pub type SqlitePooledConnection = PooledConnection<ConnectionManager<SqliteConnection>>;
 pub const MIGRATIONS: diesel_migrations::EmbeddedMigrations = diesel_migrations::embed_migrations!("./migrations");



pub fn establish_connection() -> DbPool {
    dotenv().ok();

    if cfg!(test) {
        let manager = ConnectionManager::<SqliteConnection>::new(":memory:");
        let pool = Pool::builder().build(manager).expect("Failed to create DB pool.");

        //to do: run migrations
        
        let mut connection = pool.get().expect("Failed to get a connection from the pool");
        
        //to do: fix migration on tests
        
        pool
    } else {
    
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<SqliteConnection>::new(database_url);
        
        let pool = Pool::builder().build(manager).expect("Failed to create DB pool.");
        pool
    }
}