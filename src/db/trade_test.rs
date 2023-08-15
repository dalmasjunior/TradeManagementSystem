use crate::db::establish_connection;

fn create_wallet() -> String {
    let conn = establish_connection();
    let wallet = Wallet::create(&conn).unwrap();
    wallet.id
}

fn create_user() -> User {
    let conn = establish_connection();
    let name = Some("test_user");
    let email = Some("test_email");
    let password = Some("test_password");
    let wallet_id = create_wallet();

    let user = User::create(&conn, name, email, wallet_id, password).unwrap();
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