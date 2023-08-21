use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};

use crate::middleware::jwt_guard::JwtGuard;

use crate::db::{DbPool, models::user::User, models::wallet::Wallet};

#[derive(Serialize, Deserialize)]
pub struct UserForm {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

pub async fn create_user(user: web::Json<UserForm>, pool: web::Data<DbPool>) -> HttpResponse {
    let conn = &mut pool.get().unwrap();
    let wallet = Wallet::create(conn);
    if wallet.is_none() {
        return HttpResponse::InternalServerError().json("Failed to create wallet");
    }

    let (user, errors) = User::create(conn, user.0.name.clone(), user.0.email.clone(), wallet.unwrap().id, user.0.password.clone());
    if errors.is_some() {
        return HttpResponse::InternalServerError().json(errors.unwrap());
    } else {
        return HttpResponse::Ok().json(user);
    }
    
}

pub async fn index(pool: web::Data<DbPool>) -> HttpResponse {
    let conn = &mut pool.get().unwrap();
    let users = User::list(conn);
    if users.is_empty() {
        HttpResponse::InternalServerError().json("Failed to get users")
    } else {
        HttpResponse::Ok().json(users)
    }
}

pub async fn get(pool: web::Data<DbPool>, user_id: web::Path<String>) -> HttpResponse {
    let conn = &mut pool.get().unwrap();
    match User::find_by_id(conn, user_id.into_inner()) {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::InternalServerError().json("Failed to get user")
    }
}

pub async fn delete(pool: web::Data<DbPool>, user_id: web::Path<String>) -> HttpResponse {
    let conn = &mut pool.get().unwrap();
    match User::delete(conn, user_id.into_inner()) {
        true => HttpResponse::Ok().json("deleted"),
        false => HttpResponse::InternalServerError().json("Failed to delete user")
    }
}

pub async fn login(pool: web::Data<DbPool>, user: web::Json<LoginForm>) -> HttpResponse {
    let conn = &mut pool.get().unwrap();
    match User::login(conn, user.0.email.clone(), user.0.password.clone()) {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::InternalServerError().json("Failed to login")
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/user")
            .route(web::post().to(create_user))
            .route(web::get().to(index).wrap(JwtGuard))
    )
    .service(
        web::resource("/user/{user_id}")
            .route(web::get().to(get)).wrap(JwtGuard)
            .route(web::delete().to(delete).wrap(JwtGuard))
    )
    .service(
        web::resource("/login")
            .route(web::post().to(login))
    );
}