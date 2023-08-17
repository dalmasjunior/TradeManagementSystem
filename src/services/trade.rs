use actix_service::boxed::service;
use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};

use crate::db::{DbPool, models::Trade};

#[derive(Serialize, Deserialize)]
pub struct TradeForm {
    pub user_id: String,
    pub wallet_id: String,
    pub amount: f32,
    pub chain: String,
    pub trade_type: String,
    pub asset: String,
    pub before_price: Option<f32>,
    pub execution_price: Option<f32>,
    pub final_price: Option<f32>,
    pub traded_amount: Option<f32>,
    pub execution_fee: Option<f32>,
    pub transaction_fee: Option<f32>,
}

#[derive(Serialize, Deserialize)]
pub struct TradeQuery {
    pub start_date: String,
    pub end_date: String,
    pub trader_id: String,
}

fn fill_optional_fields(trade: web::Json<TradeForm>) -> Trade {
    Trade {
        user_id: trade.0.user_id.clone(),
        wallet_id: trade.0.wallet_id.clone(),
        amount: trade.0.amount,
        chain: trade.0.chain.clone(),
        trade_type: trade.0.trade_type.clone(),
        asset: trade.0.asset.clone(),
        before_price: if trade.0.before_price.is_none() {0.0} else {trade.0.before_price.unwrap()},
        execution_price: if trade.0.execution_price.is_none() {0.0} else {trade.0.execution_price.unwrap()},
        final_price: if trade.0.final_price.is_none() {0.0} else {trade.0.final_price.unwrap()},
        traded_amount: if trade.0.traded_amount.is_none() {0.0} else {trade.0.traded_amount.unwrap()},
        execution_fee: if trade.0.execution_fee.is_none() {0.0} else {trade.0.execution_fee.unwrap()},
        transaction_fee: if trade.0.transaction_fee.is_none() {0.0} else {trade.0.transaction_fee.unwrap()},
        id: "".to_string(),
        created_at: chrono::Local::now().naive_local(),
        updated_at: chrono::Local::now().naive_local(),
    }
}

pub async fn create_trade(trade: web::Json<TradeForm>, pool: web::Data<DbPool>) -> HttpResponse {
    let conn = &mut pool.get().unwrap();
    let mut trade = fill_optional_fields(trade);
    match Trade::create(conn, &mut trade) {
        Some(trade) => HttpResponse::Ok().json(trade),
        None => HttpResponse::InternalServerError().into()
    }
}

pub async fn index(pool: web::Data<DbPool>) -> HttpResponse {
    let conn = &mut pool.get().unwrap();
    let trades = Trade::list(conn);
    if trades.is_empty() {
        HttpResponse::InternalServerError().into()
    } else {
        HttpResponse::Ok().json(trades)
    }
}

pub async fn get(pool: web::Data<DbPool>, trade_id: web::Path<String>) -> HttpResponse {
    let conn = &mut pool.get().unwrap();
    match Trade::find_by_id(conn, trade_id.into_inner()) {
        Some(trade) => HttpResponse::Ok().json(trade),
        None => HttpResponse::InternalServerError().into()
    }
}

pub async fn update(pool: web::Data<DbPool>, trade_id: web::Path<String>, trade: web::Json<TradeForm>) -> HttpResponse {
    let conn = &mut pool.get().unwrap();
    let mut trade = fill_optional_fields(trade);
    match Trade::update(conn, trade_id.into_inner(), &mut trade) {
        Some(trade) => HttpResponse::Ok().json(trade),
        None => HttpResponse::InternalServerError().into()
    }
}

pub async fn delete(pool: web::Data<DbPool>, trade_id: web::Path<String>) -> HttpResponse {
    let conn = &mut pool.get().unwrap();
    match Trade::delete(conn, trade_id.into_inner()) {
        true => HttpResponse::Ok().into(),
        false => HttpResponse::InternalServerError().into()
    }
}

pub async fn profit_loss(pool: web::Data<DbPool>, params: web::Query<TradeQuery>) -> HttpResponse {
    let conn = &mut pool.get().unwrap();
    let trades = Trade::list(conn);
    if trades.is_empty() {
        HttpResponse::InternalServerError().into()
    } else {
        HttpResponse::Ok().json(trades)
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/trade")
            .route(web::post().to(create_trade))
            .route(web::get().to(index))
    )
    .service(
        web::resource("/trade/{trade_id}")
            .route(web::get().to(get))
            .route(web::put().to(update))
            .route(web::delete().to(delete))
    )
    .service(
        web::resource("/trade/profit-loss")
        .route(web::get().to(profit_loss))
    );
}