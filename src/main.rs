extern crate diesel;
extern crate diesel_migrations;
extern crate serde_json;
extern crate r2d2_diesel;

mod utils;
mod db;
mod services;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer, web::JsonConfig};

    let conn_pool = db::establish_connection();

    HttpServer::new(move || {
        App::new()
            .app_data(conn_pool.clone())
            .app_data(JsonConfig::default().limit(4096))
            .configure(services::user::init_routes)
    })
    .bind(("0.0.0.0", 5000))?
    .run()
    .await    
}
