//! This module defines various structs and utility methods related to trading activities and statistics.
//!
//! It includes the definition of the `Trade` struct, which represents a trading activity with various attributes such as
//! trade ID, user ID, wallet ID, trade amount, trade chain, trade type, asset, and timestamps for creation and update.
//! It also includes definitions of several supporting data structures for representing daily profit/loss, cumulative fees, slippage, etc.
//! 
//! The module provides methods for interacting with trade data, calculating various statistics such as profit/loss and slippage,
//! and validating the integrity of trade attributes like trade chain, trade type, and asset.
//! 
//! Additionally, it offers utilities for categorizing trade statistics by various dimensions like asset or trade type,
//! as well as methods for retrieving and manipulating trade records in the database.
//! 
//! # Examples
//! 
//! ```rust
//! use crate::models::trade::{Trade, DailyProfitLoss, CumulativeFeesResponse, SlippageByTrader};
//!
//! // List all trades in the database
//! let trades = Trade::list(&mut connection);
//!
//! // Find a trade by ID
//! if let Some(trade) = Trade::find_by_id(&mut connection, "trade_id".to_string()) {
//!     println!("Found trade: {:?}", trade);
//! }
//!
//! // Create a new trade
//! let mut new_trade = Trade::create(&mut connection, &mut Trade { /* trade attributes */ });
//! if let Some(new_trade) = new_trade {
//!     println!("Created new trade: {:?}", new_trade);
//! }
//!
//! // Update trade information
//! if let Some(updated_trade) = Trade::update(&mut connection, "trade_id".to_string(), &mut Trade { /* updated trade attributes */ }) {
//!     println!("Updated trade: {:?}", updated_trade);
//! }
//!
//! // Delete a trade
//! if Trade::delete(&mut connection, "trade_id".to_string()) {
//!     println!("Trade deleted");
//! }
//!
//! // Calculate cumulative fees for a specific date range and user
//! let cumulative_fees = Trade::cumulative_fees(&mut connection, "start_date".to_string(), "end_date".to_string(), "user_id".to_string());
//! println!("Cumulative fees: {:?}", cumulative_fees);
//!
//! // Calculate daily profit/loss for a specific date range, user, and optionally by asset or trade type
//! let profit_loss = Trade::profit_loss(&mut connection, "start_date".to_string(), "end_date".to_string(), "user_id".to_string(), Some("asset".to_string()), None);
//! println!("Daily profit/loss: {:?}", profit_loss);
//!
//! // Calculate slippage statistics for a specific date range and user
//! let slippage_stats = Trade::get_slippage_bt_dates(&mut connection, "start_date".to_string(), "end_date".to_string(), "user_id".to_string());
//! println!("Slippage statistics: {:?}", slippage_stats);
//! ```
//!
//! # Note
//! This module assumes the availability of a database connection (`SqliteConnection` in this case) for trade data retrieval and manipulation.
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;

use super::super::schema::{*, self};
use super::super::schema::trades::dsl::trades as trades_dsl;

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[diesel(table_name = crate::db::schema::trades)]
pub struct Trade {
    pub id: String,
    pub user_id: String,
    pub wallet_id: String,
    pub amount: f32,
    pub chain: String,
    pub trade_type: String,
    pub asset: String,
    pub before_price: f32,
    pub execution_price: f32,
    pub final_price: f32,
    pub traded_amount: f32,
    pub execution_fee: f32,
    pub transaction_fee: f32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct DailyProfitLoss {
    pub date: String,
    pub profit: f32,
    pub loss: f32,
}

#[derive(Serialize, Deserialize)]
pub struct CumulativeFeesResponse {
    pub trader_id: String,
    pub cumulative_fees: f32,
}

#[derive(Serialize, Deserialize)]
pub struct DailyProfitLossByAsset {
    pub date: String,
    pub profit: f32,
    pub loss: f32,
    pub asset: String,
}

#[derive(Serialize, Deserialize)]
pub struct DailyProfitLossByTradeType {
    pub date: String,
    pub profit: f32,
    pub loss: f32,
    pub trade_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct SlippageByTrader {
    pub trader_id: String,
    pub total_slippage: f32,
    pub average_slippage: f32,
    pub total_slippage_cost_percent: f32,
    pub average_slippage_cost_percent: f32    
}

pub struct Chain;
pub struct TradeType;
pub struct Asset;

impl Chain {
    pub fn is_valid(chain: &str) -> bool {
        match chain {
            "Ethereum" => true,
            "Arbitrum" => true,
            "Optimism" => true,
            "Polygon" => true,
            _ => false,
        }
    }    
}

impl TradeType {
    pub fn is_valid(tradetype: &str) -> bool {
        match tradetype {
            "LimitBuy" => true,
            "LimitSell" => true,
            "MarketBuy" => true,
            "MarketSell" => true,
            _ => false,
        }
    }
}

impl Asset {
    pub fn is_valid(asset: &str) -> bool {
        match asset {
            "BTC" => true,
            "ETH" => true,
            "XRP" => true,
            "XLM" => true,
            "DOGE" => true,
            _ => false,
        }
    }
}

impl Trade {
    

    pub fn list(conn: &mut SqliteConnection) -> Vec<Self> {
        trades_dsl
            .order(trades::id.desc())
            .load::<Trade>(conn)
            .expect("Error loading wallets")
    }

    pub fn find_by_id(conn: &mut SqliteConnection, id: String) -> Option<Self> {
        if let Ok(record) = trades_dsl
            .find(id)
            .get_result::<Trade>(conn) {
            Some(record)
            } else {
                None
            }
    }

    pub fn create(conn: &mut SqliteConnection, trade: &mut Self) -> Option<Self> {
        trade.id = Uuid::new_v4().as_hyphenated().to_string();
        
        if trade.chain.is_empty() || trade.trade_type.is_empty() || trade.asset.is_empty() {
            return None;
        }
        
        if !Chain::is_valid(&trade.chain) || !TradeType::is_valid(&trade.trade_type) || !Asset::is_valid(&trade.asset) {
            return None;
        }
        
        diesel::insert_into(trades_dsl)
            .values(&*trade)
            .execute(conn)
            .expect("Error saving new trade");
        
        Self::find_by_id(conn, trade.id.clone())
    }

    pub fn update(conn: &mut SqliteConnection, id: String, trade: &mut Trade) -> Option<Self> {
        if trade.chain.is_empty() || trade.trade_type.is_empty() || trade.asset.is_empty() {
            return None;
        }

        diesel::update(trades_dsl.find(id.clone()))
            .set((
                schema::trades::amount.eq(trade.amount.clone()),
                schema::trades::chain.eq(trade.chain.clone()),
                schema::trades::trade_type.eq(trade.trade_type.clone()),
                schema::trades::asset.eq(trade.asset.clone()),
                schema::trades::before_price.eq(trade.before_price.clone()),
                schema::trades::execution_price.eq(trade.execution_price.clone()),
                schema::trades::final_price.eq(trade.final_price.clone()),
                schema::trades::traded_amount.eq(trade.traded_amount.clone()),
                schema::trades::execution_fee.eq(trade.execution_fee.clone()),
                schema::trades::transaction_fee.eq(trade.transaction_fee.clone()),
                schema::trades::updated_at.eq(chrono::Local::now().naive_local())))
            .execute(conn)
            .expect("Error updating trade");
        
        Self::find_by_id(conn, id)
    }

    pub fn delete(conn: &mut SqliteConnection, id: String) -> bool {
        diesel::delete(trades_dsl.find(id.clone()))
            .execute(conn)
            .expect("Error deleting trade");
        
        Self::find_by_id(conn, id).is_none()
    }

    fn get_dates_by_asset(conn: &mut SqliteConnection,start_date: String, end_date: String, user_id: String, asset: String) -> Vec<Self> {
        trades_dsl
            .filter(trades::user_id.eq(user_id))
            .filter(trades::created_at.ge(start_date))
            .filter(trades::created_at.le(end_date))
            .filter(trades::asset.eq(asset))
            .load::<Trade>(conn)
            .expect("Error loading trades")
    }

    fn get_dates_by_trade(conn: &mut SqliteConnection, start_date: String, end_date: String, user_id: String, tradetype: String) -> Vec<Self> {
        trades_dsl
            .filter(trades::user_id.eq(user_id))
            .filter(trades::created_at.ge(start_date))
            .filter(trades::created_at.le(end_date))
            .filter(trades::trade_type.eq(tradetype))
            .load::<Trade>(conn)
            .expect("Error loading trades")
    }

    fn get_bt_dates(conn: &mut SqliteConnection,start_date: String, end_date: String, user_id: String) -> Vec<Self> {
        trades_dsl
            .filter(trades::user_id.eq(user_id))
            .filter(trades::created_at.ge(start_date))
            .filter(trades::created_at.le(end_date))
            .load::<Trade>(conn)
            .expect("Error loading trades")
    }
    
    pub fn cumulative_fees(conn: &mut SqliteConnection, start_date: String, end_date: String, user_id: String) -> CumulativeFeesResponse {
        let trades: Vec<Trade> = Self::get_bt_dates(conn, start_date, end_date, user_id.clone());
        
        let mut fees = 0.0;
        for trade in trades.iter() {
            fees += trade.execution_fee + trade.transaction_fee;
        }

        CumulativeFeesResponse { trader_id: user_id, cumulative_fees: fees.round() }
    }

    pub fn profit_loss(conn: &mut SqliteConnection, start_date: String, end_date: String, user_id: String, asset: Option<String>, tradetype: Option<String>) -> Vec<DailyProfitLoss> {
        let trades: Vec<Trade>;
        if asset.is_some() {
            trades = Self::get_dates_by_asset(conn, start_date, end_date, user_id, asset.unwrap());
        } else if tradetype.is_some() {
            trades = Self::get_dates_by_trade(conn, start_date, end_date, user_id, tradetype.unwrap());
        } else {
            trades = Self::get_bt_dates(conn, start_date, end_date, user_id);
        }
        
        let mut daily_profit_loss: Vec<DailyProfitLoss> = Vec::new();
        let mut dates: Vec<String> = Vec::new();
        for trade in trades.iter() {
            if !dates.contains(&trade.created_at.date().to_string()) {
                dates.push(trade.created_at.date().to_string());
            }
        };
        for date in dates {
            let mut profit = 0.0;
            let mut loss = 0.0;
            for trade in trades.iter() {
                if trade.created_at.date().to_string() == date {
                    let pnl = trade.calculate_trade_pnl();
                    if pnl > 0.0 {
                        profit += pnl;
                    } else {
                        loss += pnl;
                    }
                }
            }
            daily_profit_loss.push(DailyProfitLoss {
                date: date,
                profit: profit.round(),
                loss: loss.round(),
            });
        }
        daily_profit_loss
    }

    fn calculate_trade_pnl(&self) -> f32{
        let pnl : f32;

        if self.trade_type == "LimitBuy" || self.trade_type == "MarketBuy" {
           pnl = self.final_price - self.execution_price;
        } else if self.trade_type == "LimitSell" || self.trade_type == "MarketSell" {
            pnl = self.final_price - self.before_price;
        } else {
            pnl = 0.0;
        }

        pnl * self.traded_amount - self.execution_fee - self.transaction_fee
    }

    pub fn get_slippage_bt_dates(conn: &mut SqliteConnection, start_date: String, end_date: String, user_id: String) -> SlippageByTrader {
        let trades = Trade::get_bt_dates(conn, start_date, end_date, user_id.clone());
        
        let mut total_slippage = 0.0;
        let mut total_slippage_cost_percent = 0.0;
        
        for trade in &trades {
            let (slippage, slippage_cost_percent) = trade.calculate_slippage();
            total_slippage += slippage;
            total_slippage_cost_percent += slippage_cost_percent;
        };
        
        let average_slippage = total_slippage / trades.len() as f32;
        let average_slippage_cost_percent = total_slippage_cost_percent / trades.len() as f32;
        
        SlippageByTrader {
            trader_id: user_id,
            total_slippage: total_slippage.round(),
            average_slippage: average_slippage.round(),
            total_slippage_cost_percent: total_slippage_cost_percent.round(),
            average_slippage_cost_percent: average_slippage_cost_percent.round(),
        }

    }

    pub fn calculate_slippage(&self) -> (f32, f32) {
        let total_execution_cost = self.execution_price * self.traded_amount;
        let total_fees = self.execution_fee + self.transaction_fee;
        let effective_price = (total_execution_cost + total_fees) / self.traded_amount;

        let slippage = effective_price - self.before_price;
        let slippage_cost_percent = (slippage / self.before_price) * 100.00;
        
        (slippage, slippage_cost_percent)
    } 
}


