//! This module defines a collection of functions and structs for managing trade-related operations
//! using the Actix Web framework, along with various utility libraries such as `serde`, `chrono`, and more.
//!
//! The provided functions include:
//!
//! - `create_trade`: Handles the creation of a new trade entry in the database.
//! - `index`: Retrieves a list of all trades from the database.
//! - `get`: Retrieves a specific trade entry by its ID.
//! - `update`: Updates a specific trade entry with new information.
//! - `delete`: Deletes a specific trade entry from the database.
//! - `profit_loss`: Calculates and retrieves profit and loss data for trades within a specified date range.
//! - `cumulative_fee`: Calculates and retrieves cumulative fee data for trades within a specified date range.
//! - `slippage`: Retrieves slippage data for trades within a specified date range.
//! - `init_routes`: Initializes routes for handling trade-related HTTP requests.
//!
//! # Examples
//!
//! ```
//! use actix_web::{web, HttpResponse};
//! use serde::{Deserialize, Serialize};
//!
//! // ... imports ...
//!
//! /// Represents a form for creating a new trade.
//! #[derive(Serialize, Deserialize)]
//! pub struct TradeForm {
//!     // ... fields ...
//! }
//!
//! // ... other structs ...
//!
//! /// Fills in optional fields of a trade form to create a `Trade` object.
//! fn fill_optional_fields(trade: &TradeForm) -> Trade {
//!     // ... implementation details ...
//! }
//!
//! /// Handles the creation of a new trade entry in the database.
//! pub async fn create_trade(trade: web::Json<TradeForm>, pool: web::Data<DbPool>) -> HttpResponse {
//!     // ... implementation details ...
//! }
//!
//! // ... other functions ...
//!
//! /// Initializes routes for handling trade-related HTTP requests.
//! pub fn init_routes(cfg: &mut web::ServiceConfig) {
//!     // ... route configuration ...
//! }
//! ```
//!
//! # Note
//!
//! Some of the functions in this module require authentication through JSON Web Tokens (JWT),
//! and they are wrapped with the `JwtGuard` middleware for secure access.

use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::{
    db::{models::trade::Trade, DbPool},
    middleware::jwt_guard::JwtGuard, utils,
};

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
    pub timestamp: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct TradeQuery {
    pub start_date: String,
    pub end_date: String,
    pub trader_id: String,
    pub asset: Option<String>,
    pub trade_type: Option<String>,
}

pub fn fill_optional_fields(trade: &TradeForm) -> Trade {
    Trade {
        user_id: trade.user_id.clone(),
        wallet_id: trade.wallet_id.clone(),
        amount: trade.amount,
        chain: trade.chain.clone(),
        trade_type: trade.trade_type.clone(),
        asset: trade.asset.clone(),
        before_price: if trade.before_price.is_none() {
            0.0
        } else {
            trade.before_price.unwrap()
        },
        execution_price: if trade.execution_price.is_none() {
            0.0
        } else {
            trade.execution_price.unwrap()
        },
        final_price: if trade.final_price.is_none() {
            0.0
        } else {
            trade.final_price.unwrap()
        },
        traded_amount: if trade.traded_amount.is_none() {
            0.0
        } else {
            trade.traded_amount.unwrap()
        },
        execution_fee: (trade.execution_price.unwrap_or(0.0) * trade.traded_amount.unwrap_or(0.0)) * 0.003,
        transaction_fee: trade.execution_price.unwrap_or(0.0) * 0.005,
        id: "".to_string(),
        created_at: if trade.timestamp.is_none() {
            chrono::Local::now().naive_local()
        } else {
            utils::date::timestamp_to_naive_date_time(trade.timestamp.unwrap())
        },
        updated_at: chrono::Local::now().naive_local(),
    }
}

pub async fn create_trade(trade: web::Json<TradeForm>, pool: web::Data<DbPool>) -> HttpResponse {
    let conn = &mut pool.get().unwrap();
    
    let mut trade = fill_optional_fields(&trade.0);
    match Trade::create(conn, &mut trade) {
        Some(trade) => HttpResponse::Ok().json(trade),
        None => HttpResponse::InternalServerError().into(),
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
        None => HttpResponse::InternalServerError().into(),
    }
}

pub async fn update(
    pool: web::Data<DbPool>,
    trade_id: web::Path<String>,
    trade: web::Json<TradeForm>,
) -> HttpResponse {
    let conn = &mut pool.get().unwrap();
    let mut trade = fill_optional_fields(&trade.0);
    match Trade::update(conn, trade_id.into_inner(), &mut trade) {
        Some(trade) => HttpResponse::Ok().json(trade),
        None => HttpResponse::InternalServerError().into(),
    }
}

pub async fn delete(pool: web::Data<DbPool>, trade_id: web::Path<String>) -> HttpResponse {
    let conn = &mut pool.get().unwrap();
    match Trade::delete(conn, trade_id.into_inner()) {
        true => HttpResponse::Ok().into(),
        false => HttpResponse::InternalServerError().into(),
    }
}

pub async fn profit_loss(pool: web::Data<DbPool>, params: web::Query<TradeQuery>) -> HttpResponse {
    let conn = &mut pool.get().unwrap();

    if params.start_date.is_empty() || params.end_date.is_empty() || params.trader_id.is_empty() {
        return HttpResponse::BadRequest()
            .json("Error: Start date, End date and Trader ID are required");
    }

    let trades = Trade::profit_loss(
        conn,
        params.start_date.clone(),
        params.end_date.clone(),
        params.trader_id.clone(),
        params.asset.clone(),
        params.trade_type.clone(),
    );

    HttpResponse::Ok().json(trades)
}

pub async fn cumulative_fee(
    pool: web::Data<DbPool>,
    params: web::Query<TradeQuery>,
) -> HttpResponse {
    let conn = &mut pool.get().unwrap();

    if params.start_date.is_empty() || params.end_date.is_empty() || params.trader_id.is_empty() {
        return HttpResponse::BadRequest().json("Error: Start date, End date and Trader ID are required")
    }

    let fees = Trade::cumulative_fees(
        conn,
        params.start_date.clone(),
        params.end_date.clone(),
        params.trader_id.clone(),
    );

    HttpResponse::Ok().json(fees)
}

pub async fn slippage(pool: web::Data<DbPool>, params: web::Query<TradeQuery>) -> HttpResponse {
    let conn = &mut pool.get().unwrap();
    
    if params.start_date.is_empty() || params.end_date.is_empty() || params.trader_id.is_empty() {
        return HttpResponse::BadRequest()
            .json("Error: Start date, End date and Trader ID are required");
    }

    let slippage = Trade::get_slippage_bt_dates(
        conn,
        params.start_date.clone(),
        params.end_date.clone(),
        params.trader_id.clone(),
    );

    HttpResponse::Ok().json(slippage)
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/trade")
            .route(web::post().to(create_trade).wrap(JwtGuard))
            .route(web::get().to(index).wrap(JwtGuard)),
    )
    .service(
        web::resource("/trade/{trade_id}")
            .route(web::get().to(get).wrap(JwtGuard))
            .route(web::put().to(update).wrap(JwtGuard))
            .route(web::delete().to(delete).wrap(JwtGuard)),
    )
    .service(web::resource("/profit-loss").route(web::get().to(profit_loss).wrap(JwtGuard)))
    .service(web::resource("/cumulative-fees").route(web::get().to(cumulative_fee).wrap(JwtGuard)))
    .service(web::resource("/slippage").route(web::get().to(slippage).wrap(JwtGuard)));
}
