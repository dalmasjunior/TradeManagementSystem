use uuid::Uuid;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;

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

    pub fn create(conn: &mut SqliteConnection, user_id: String, wallet_id: String, amount: f32, chain: String, trade_type: String, asset: String, before_price: f32, execution_price: f32, final_price: f32, traded_amount: f32, execution_fee: f32, transaction_fee: f32) -> Option<Self> {
        let new_id = Uuid::new_v4().as_hyphenated().to_string();

        if chain.is_empty() || trade_type.is_empty() || asset.is_empty() {
            return None;
        }

        let new_trade = Self::new_trade_struct(new_id, user_id, wallet_id, amount, chain, trade_type, asset, before_price, execution_price, final_price, traded_amount, execution_fee, transaction_fee);

        diesel::insert_into(trades_dsl)
            .values(&new_trade)
            .execute(conn)
            .expect("Error saving new trade");
        
        Self::find_by_id(conn, new_trade.id)
    }

    fn new_trade_struct(id: String, user_id: String, wallet_id: String, amount: f32, chain: String, trade_type: String, asset: String, before_price: f32, execution_price: f32, final_price: f32, traded_amount: f32, execution_fee: f32, transaction_fee: f32) -> Self {
        Self {
            id: id,
            user_id: user_id,
            wallet_id: wallet_id,
            amount: amount,
            chain: chain,
            trade_type: trade_type,
            asset: asset,
            before_price: before_price,
            execution_price: execution_price,
            final_price: final_price,
            traded_amount: traded_amount,
            execution_fee: execution_fee,
            transaction_fee: transaction_fee,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }

    pub fn update(conn: &mut SqliteConnection, id: String, amount: f32, chain: String, trade_type: String, asset: String, before_price: f32, execution_price: f32, final_price: f32, traded_amount: f32, execution_fee: f32, transaction_fee: f32) -> Option<Self> {
        if chain.is_empty() || trade_type.is_empty() || asset.is_empty() {
            return None;
        }

        diesel::update(trades_dsl.find(id.clone()))
            .set((
                schema::trades::amount.eq(amount),
                schema::trades::chain.eq(chain),
                schema::trades::trade_type.eq(trade_type),
                schema::trades::asset.eq(asset),
                schema::trades::before_price.eq(before_price),
                schema::trades::execution_price.eq(execution_price),
                schema::trades::final_price.eq(final_price),
                schema::trades::traded_amount.eq(traded_amount),
                schema::trades::execution_fee.eq(execution_fee),
                schema::trades::transaction_fee.eq(transaction_fee),
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
}

#[cfg(test)]
mod user_test;
#[cfg(test)]
mod trade_test;
#[cfg(test)]
mod wallet_test;