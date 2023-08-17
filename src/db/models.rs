


use diesel::internal::table_macro::{SelectStatement, FromClause};

use diesel::sql_types::SqlType;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;


use super::schema::trades::trade_type;
use super::schema::{*, self};
use super::schema::trades::dsl::trades as trades_dsl;
use super::schema::users::dsl::users as users_dsl;
use super::schema::wallet::dsl::{
    wallet as wallet_dsl, 
    balance as balance_dsl,
    hash as hash_dsl,
};

use crate::utils::hash::new_hash;

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[diesel(table_name = crate::db::schema::wallet)]
pub struct Wallet {
    pub id: String,
    pub hash: String,
    pub balance: f32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[diesel(table_name = crate::db::schema::users)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub wallet_id: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

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

enum DailyResultType {
    Asset(Vec<DailyProfitLossByAsset>),
    TradeType(Vec<DailyProfitLossByTradeType>)
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
    pub fn is_valid(trade_type: &str) -> bool {
        match trade_type {
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

impl Wallet {
    pub fn list(conn: &mut SqliteConnection) -> Vec<Self> {
        wallet_dsl
            .order(wallet::id.desc())
            .load::<Wallet>(conn)
            .expect("Error loading wallets")
    }
    
    pub fn find_by_id(conn: &mut SqliteConnection, id: String) -> Option<Self> {
        if let Ok(record) = wallet_dsl
            .find(id)
            .get_result::<Wallet>(conn) {
            Some(record)
            } else {
                None
            }
    }

    pub fn find_by_hash(conn: &mut SqliteConnection, hash: String) -> Option<Self> {
        let wallet = wallet_dsl
            .filter(hash_dsl.eq(hash))
            .first::<Wallet>(conn)
            .optional()
            .expect("Error loading wallet");

        match wallet {
            Some(wallet) => Some(wallet),
            None => None,
        }
    }

    pub fn create(conn: &mut SqliteConnection) -> Option<Self> {
        let new_id = Uuid::new_v4().as_hyphenated().to_string();
        let new_hash = new_hash();
        let new_wallet = Self::new_wallet_struct(new_id, new_hash.clone(), 0.0);

        diesel::insert_into(wallet_dsl)
            .values(&new_wallet)
            .execute(conn)
            .expect("Error saving new wallet");
        
        Self::find_by_hash(conn, new_hash)
    }

    fn new_wallet_struct(id: String, hash: String, balance: f32) -> Self {
        Self {
            id: id,
            hash: hash,
            balance: balance,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }

    pub fn update_balance(conn: &mut SqliteConnection, id: String, balance: f32) -> Option<Self> {
        if let Some(mut _wallet) = Self::find_by_id(conn, id.clone()) {
            diesel::update(wallet_dsl.find(id.clone()))
                .set(balance_dsl.eq(balance))
                .execute(conn)
                .expect("Error updating wallet");
            Self::find_by_id(conn, id)
        } else {
            None
        }
    }
}

impl User {
    pub fn list(conn: &mut SqliteConnection) -> Vec<Self> {
        users_dsl
            .order(users::id.desc())
            .load::<User>(conn)
            .expect("Error loading users")
    }

    pub fn find_by_id(conn: &mut SqliteConnection, id: String) -> Option<Self> {
        if let Ok(record) = users_dsl
            .find(id)
            .get_result::<User>(conn) {
            Some(record)
            } else {
                None
            }
    }

    pub fn find_by_email(conn: &mut SqliteConnection, email: String) -> Option<Self> {
        if let Ok(record) = users_dsl
            .filter(users::email.eq(email))
            .get_result::<User>(conn) {
            Some(record)
            } else {
                None
            }
    }

    pub fn create(conn: &mut SqliteConnection, name: String, email: String, wallet_id: String, password: String) -> Option<Self> {
        let new_id = Uuid::new_v4().as_hyphenated().to_string();

        if email.is_empty() || password.is_empty() || name.is_empty() || wallet_id.is_empty() {
            return None;
        }
        
        
        if Self::find_by_email(conn, email.clone()).is_some() {
            return None;
        }
        
        
        if Wallet::find_by_id(conn, wallet_id.clone()).is_none() {
            return None;
        }
        
        let new_user = Self::new_user_struct(new_id, name, email, wallet_id, password);

        diesel::insert_into(users_dsl)
            .values(&new_user)
            .execute(conn)
            .expect("Error saving new user");
        
        Self::find_by_id(conn, new_user.id)
    }

    fn new_user_struct(id: String, name: String, email: String, wallet_id: String, password: String) -> Self {
        Self {
            id: id,
            name: name,
            email: email,
            password: password,
            wallet_id: wallet_id,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }

    pub fn update(conn: &mut SqliteConnection, id: String, name: String, email: String, wallet: String, password: String) -> Option<Self> {
        if let Ok(record) = users_dsl
            .find(id)
            .get_result::<User>(conn) {
            let updated_user = Self::update_user_struct(record, name, email, wallet, password);
            diesel::update(users_dsl.find(updated_user.id.clone()))
                .set((schema::users::name.eq(updated_user.name.clone()),
                    schema::users::email.eq(updated_user.email.clone()),
                    schema::users::wallet_id.eq(updated_user.wallet_id.clone()),
                    schema::users::password.eq(updated_user.password.clone()),
                    schema::users::updated_at.eq(chrono::Local::now().naive_local())))
                .execute(conn)
                .expect("Error updating user");
            Some(updated_user)
            } else {
                None
            }
    }

    fn update_user_struct(mut user: Self, name: String, email: String, wallet: String, password: String) -> Self {
        user.name = name;
        user.email = email;
        user.wallet_id = wallet;
        user.password = password;
        user.updated_at = chrono::Local::now().naive_local();
        user
    }

    pub fn delete(conn: &mut SqliteConnection, id: String) -> bool {
        if let Ok(_record) = users_dsl
            .find(id.clone())
            .get_result::<User>(conn) {
            diesel::delete(users_dsl.find(id))
                .execute(conn)
                .expect("Error deleting user");
            true
            } else {
                false
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

    pub fn create(conn: &mut SqliteConnection, trade: &mut Trade) -> Option<Self> {
        trade.id = Uuid::new_v4().as_hyphenated().to_string();
        
        if trade.chain.is_empty() || trade.trade_type.is_empty() || trade.asset.is_empty() {
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
    
    pub fn cumulativeFees(conn: &mut SqliteConnection, start_date: String, end_date: String, user_id: String) -> f32 {
        let trades: Vec<Trade> = Self::get_bt_dates(conn, start_date, end_date, user_id);

        let mut fees = 0.0;
        for trade in trades.iter() {
            fees += trade.execution_fee + trade.transaction_fee;
        }
        fees
    }

    pub fn profit_loss(conn: &mut SqliteConnection, start_date: String, end_date: String, user_id: String, asset: Option<String>, tradetype: Option<String>) -> Vec<DailyProfitLoss> {
        let mut trades: Vec<Trade>;
        if asset.is_some() {
            trades = Self::get_dates_by_asset(conn, start_date, end_date, user_id, asset.unwrap());
        } else if tradetype.is_some() {
            trades = Self::get_dates_by_trade(conn, start_date, end_date, user_id, tradetype.unwrap());
        } else {
            trades = Self::get_bt_dates(conn, start_date, end_date, user_id);
        }
        
        let mut daily_profit_loss: Vec<DailyProfitLoss> = Vec::new();
        let mut dates: Vec<String> = Vec::new();
        for trade in trades {
            if !dates.contains(&trade.created_at.date().to_string()) {
                dates.push(trade.created_at.date().to_string());
            }
        };
        for date in dates {
            let mut profit = 0.0;
            let mut loss = 0.0;
            for trade in trades.iter() {
                if trade.created_at.date().to_string() == date {
                    let pnl = Self::calculate_trade_pnl(trade);
                    if pnl > 0.0 {
                        profit += pnl;
                    } else {
                        loss += pnl;
                    }
                }
            }
            daily_profit_loss.push(DailyProfitLoss {
                date: date,
                profit: profit,
                loss: loss,
            });
        }
        daily_profit_loss
    }

    fn calculate_trade_pnl(trade: &Trade) -> f32{
        let pnl : f32;

        if trade.trade_type == "LimitBuy" || trade.trade_type == "MarketBuy" {
           pnl = trade.final_price - trade.execution_price;
        } else if trade.trade_type == "LimitSell" || trade.trade_type == "MarketSell" {
            pnl = trade.final_price - trade.before_price;
        } else {
            pnl = 0.0;
        }

        pnl * trade.traded_amount - trade.execution_fee - trade.transaction_fee
    }
}

#[cfg(test)]
mod user_test;
#[cfg(test)]
mod trade_test;
#[cfg(test)]
mod wallet_test;