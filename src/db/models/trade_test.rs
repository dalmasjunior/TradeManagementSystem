use r2d2::PooledConnection;
use rand::Rng;

use crate::db::establish_connection;
use crate::services::trade::{TradeForm, fill_optional_fields};
use super::trade::Trade;
use super::wallet::Wallet;
use super::user::User;

fn get_connection() -> PooledConnection<diesel::r2d2::ConnectionManager<diesel::SqliteConnection>> {
    let pool = establish_connection();
    pool.get().unwrap()
}

fn create_wallet() -> String {
    let conn = &mut get_connection();
    let wallet = Wallet::create(conn).unwrap();
    wallet.id
}

fn create_user() -> User {
    let conn = &mut get_connection();
    let name = "test_user".to_string();
    let email = "test_email".to_string();
    let password = "test_password".to_string();
    let wallet_id = create_wallet();

    let (user, _err) = User::create(conn, name, email, wallet_id, password);
    user.unwrap()    
}

fn gen_rand_trade(user_id: String, wallet_id: String) -> Trade {
    let mut rng = rand::thread_rng();

    let trade_form = TradeForm {
        user_id: user_id,
        wallet_id: wallet_id,
        trade_type: if rng.gen() {
            if rng.gen() {
                "LimitBuy".to_string()
            } else {
                "LimitSell".to_string()
            }
        } else {
            if rng.gen() {
                "MarketBuy".to_string()
            } else {
                "MarketSell".to_string()
            }
        },
        amount: rng.gen_range(1.0..100.0),
        chain: if rng.gen() {
            "Ethereum".to_string()
        } else {
            "Bitcoin".to_string()
        },
        asset: if rng.gen() {
            "ETH".to_string()
        } else {
            "BTC".to_string()
        },
        before_price: Some(rng.gen_range(1.0..100.0)),
        execution_price: Some(rng.gen_range(1.0..100.0)),
        final_price: Some(rng.gen_range(1.0..100.0)),
        traded_amount: Some(rng.gen_range(1.0..100.0)),
        timestamp: Some(rng.gen_range(1641045600..1672418400)),
    };

    fill_optional_fields(&trade_form)
}

#[test]
fn create_trade() {
    let conn = &mut get_connection();
    let user_id = create_user().id;
    let wallet_id = create_wallet();
    let mut new_trade = gen_rand_trade(user_id, wallet_id);
    
        
    let trade = Trade::create(conn, &mut new_trade).unwrap();

    assert_eq!(trade.user_id, new_trade.user_id);
    assert_eq!(trade.trade_type, new_trade.trade_type);
    assert_eq!(trade.wallet_id, new_trade.wallet_id);
    assert_eq!(trade.amount, new_trade.amount);
    assert_eq!(trade.chain, new_trade.chain);
    assert_eq!(trade.asset, new_trade.asset);
    assert_eq!(trade.before_price, new_trade.before_price);
    assert_eq!(trade.execution_price, new_trade.execution_price);
    assert_eq!(trade.final_price, new_trade.final_price);
    assert_eq!(trade.traded_amount, new_trade.traded_amount);
    assert_eq!(trade.execution_fee, new_trade.execution_fee);
    assert_eq!(trade.transaction_fee, new_trade.transaction_fee);

}

#[test]
fn cumulative_fees() {
    let mut _conn = get_connection();
    let user_id = create_user().id;
    let wallet_id = create_wallet();
    for _ in 0..10 {
        let mut new_trade = gen_rand_trade(user_id.clone(), wallet_id.clone());
        Trade::create(&mut _conn, &mut new_trade).unwrap();
    }
    
    let _result = Trade::profit_loss(&mut _conn, "2022-01-01".to_string(), "2023-01-08".to_string(), user_id.clone(), None, None);
    assert!(_result.len() > 0);
}

#[test]
fn cumulative_fees_by_asset() {
    let mut _conn = get_connection();
    let user_id = create_user().id;
    let wallet_id = create_wallet();
    for _ in 0..10 {
        let mut new_trade = gen_rand_trade(user_id.clone(), wallet_id.clone());
        Trade::create(&mut _conn, &mut new_trade).unwrap();
    }
    
    let _result = Trade::profit_loss(&mut _conn, "2022-01-01".to_string(), "2023-01-08".to_string(), user_id.clone(), Some("ETH".to_string()), None);
    assert!(_result.len() > 0);
}

#[test]
fn cumulative_fees_by_trade_type() {
    let mut _conn = get_connection();
    let user_id = create_user().id;
    let wallet_id = create_wallet();
    for _ in 0..10 {
        let mut new_trade = gen_rand_trade(user_id.clone(), wallet_id.clone());
        Trade::create(&mut _conn, &mut new_trade).unwrap();
    }
    
    let _result = Trade::profit_loss(&mut _conn, "2022-01-01".to_string(), "2023-01-08".to_string(), user_id.clone(), None, Some("LimitBuy".to_string()));
    assert!(_result.len() > 0);
}