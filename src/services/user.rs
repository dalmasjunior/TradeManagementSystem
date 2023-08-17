use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};

use crate::db::{DbPool, models::{User, Wallet}};

#[derive(Serialize, Deserialize)]
pub struct UserForm {
    pub name: String,
    pub email: String,
    pub password: String,
}

pub async fn create_user(user: web::Json<UserForm>, pool: web::Data<DbPool>) -> HttpResponse {
    let conn = &mut pool.get().unwrap();
    let wallet = Wallet::create(conn);
    if wallet.is_none() {
        return HttpResponse::InternalServerError().into();
    }

    match User::create(conn, user.0.name.clone(), user.0.email.clone(), wallet.unwrap().id, user.0.password.clone()) {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::InternalServerError().into()
    }
}

pub async fn index(pool: web::Data<DbPool>) -> HttpResponse {
    let conn = &mut pool.get().unwrap();
    let users = User::list(conn);
    if users.is_empty() {
        HttpResponse::InternalServerError().into()
    } else {
        HttpResponse::Ok().json(users)
    }
}

pub async fn get(pool: web::Data<DbPool>, user_id: web::Path<String>) -> HttpResponse {
    let conn = &mut pool.get().unwrap();
    match User::find_by_id(conn, user_id.into_inner()) {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::InternalServerError().into()
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/user")
            .route(web::post().to(create_user))
            .route(web::get().to(index))
    )
    .service(
        web::resource("/user/{user_id}")
            .route(web::get().to(get))
    );
}