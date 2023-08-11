mod routes {
    use actix_web::web;
    
    use crate::components::api::handlers;
    use crate::components::api::middleware;

    pub fn init_routes(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/api")
                .wrap(|req, srv| middleware::auth_middleware)
                .service(
                    web::scope("/trade")
                        .route("/execute", web::post().to(handlers::execute_trade))
                        .route("/{trade_id}", web::get().to(handlers::get_trade))
                        .route("/{trade_id}", web::put().to(handlers::update_trade)),
                ),
                
        );
        
    }
    
}