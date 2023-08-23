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
use std::error::Error;
use diesel_migrations::MigrationHarness;
use dotenv::dotenv;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;

pub mod models;
pub mod schema;

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

pub const MIGRATIONS: diesel_migrations::EmbeddedMigrations = diesel_migrations::embed_migrations!("migrations");

pub fn establish_connection() -> DbPool {
    dotenv().ok();

    if cfg!(test) {
        let manager = ConnectionManager::<SqliteConnection>::new(":memory:");
        let pool = Pool::builder().build(manager).expect("Failed to create DB pool.");
        let mut conn = pool.get().expect("Failed to get a connection from the pool");
        
        run_migrations(&mut conn).expect("Failed to run migrations");
        pool
    } else {
    
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<SqliteConnection>::new(database_url);
        
        let pool = Pool::builder().build(manager).expect("Failed to create DB pool.");
        pool
    }
}

fn run_migrations(connection: &mut SqliteConnection) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {

    // This will run the necessary migrations.
    //
    // See the documentation for `MigrationHarness` for
    // all available methods.
    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}

