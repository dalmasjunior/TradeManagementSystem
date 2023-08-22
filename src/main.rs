/// The diesel crate is used for interacting with databases.
extern crate diesel;

/// The diesel_migrations crate is used for handling database migrations.
extern crate diesel_migrations;

/// The serde_json crate is used for serializing and deserializing JSON data.
extern crate serde_json;

/// The r2d2_diesel crate is used for managing database connections.
extern crate r2d2_diesel;

/// Importing necessary components from the actix_web crate.
use actix_web::{App, HttpServer, web::{JsonConfig, Data}};
use env_logger;

/// The utils module contains utility functions and structures.
mod utils;

/// The db module contains functions and structures for database interaction.
mod db;

/// The services module contains the business logic of the application.
mod services;

/// The middleware module contains middleware functions for the application.
mod middleware;

/// The main function of the application. It sets up the server and starts it.
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Set the logging level and initialize the logger.
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    
    // Establish a connection pool to the database.
    let conn_pool = db::establish_connection();

    // Start the HTTP server.
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(conn_pool.clone())) // Share the database connection pool across the application.
            .app_data(JsonConfig::default().limit(4096)) // Configure JSON payload size limit.
            .configure(services::user::init_routes) // Configure user-related routes.
            .configure(services::trade::init_routes) // Configure trade-related routes.
    })
    .bind(("127.0.0.1", 9000))? // Bind the server to a specific address and port.
    .run()
    .await    
}
