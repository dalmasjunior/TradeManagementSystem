use r2d2::PooledConnection;

use crate::db::establish_connection;
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

    let user = User::create(conn, name, email, wallet_id, password).unwrap();
    user.id
}

#[test]
fn create_trade() {
    let conn = establish_connection();
    let user_id = create_user();
    let wallet_id = create_wallet();
    let amount = 100.0;
    let chain = "ethereum";
    let trade_type = "limitBuy";
    let asset = "ETH";
    let before_price = 100.0;
    let execution_price = 100.0;
    let final_price = 100.0;
    let traded_amount = 100.0;
    let execution_fee = 100.0;
    let transaction_fee = 100.0;
        
    let trade = Trade::create(&conn, user_id, trade_type, symbol, shares, price).unwrap();

    assert_eq!(trade.user_id, user_id);
    assert_eq!(trade.trade_type, trade_type);
    assert_eq!(trade.wallet_id, wallet_id);
    assert_eq!(trade.amount, amount);
    assert_eq!(trade.chain, chain);
    assert_eq!(trade.asset, asset);
    assert_eq!(trade.before_price, before_price);
    assert_eq!(trade.execution_price, execution_price);
    assert_eq!(trade.final_price, final_price);
    assert_eq!(trade.traded_amount, traded_amount);
    assert_eq!(trade.execution_fee, execution_fee);
    assert_eq!(trade.transaction_fee, transaction_fee);

}