use std::env;

use dotenv::dotenv;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;

pub mod models;
pub mod schema;

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;
// pub type SqlitePooledConnection = PooledConnection<ConnectionManager<SqliteConnection>>;
// pub const MIGRATIONS: diesel_migrations::EmbeddedMigrations = diesel_migrations::embed_migrations!();



pub fn establish_connection() -> DbPool {
    dotenv().ok();

    if cfg!(test) {
        let manager = ConnectionManager::<SqliteConnection>::new(":memory:");
        let pool = Pool::builder().build(manager).expect("Failed to create DB pool.");

        //to do: run migrations
        

        pool
    } else {
    
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<SqliteConnection>::new(database_url);
        
        let pool = Pool::builder().build(manager).expect("Failed to create DB pool.");
        pool
    }
}