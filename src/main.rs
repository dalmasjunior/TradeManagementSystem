extern crate diesel;
extern crate diesel_migrations;
extern crate serde_json;
extern crate r2d2_diesel;

use actix_web::{App, HttpServer, web::{JsonConfig, Data}};
use env_logger;

mod utils;
mod db;
mod services;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    

    let conn_pool = db::establish_connection();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(conn_pool.clone()))
            .app_data(JsonConfig::default().limit(4096))
            .configure(services::user::init_routes)
            .configure(services::trade::init_routes)
    })
    .bind(("127.0.0.1", 9000))?
    .run()
    .await    
}
