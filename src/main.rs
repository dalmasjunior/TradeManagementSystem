mod components;

use actix_web::{App, HttpServer};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    components::api::handlers::index();
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
        })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}