use crate::db::establish_connection;

#[test]
fn create_wallet() {
    let conn = establish_connection();
    let wallet = Wallet::create(&conn).unwrap();
    assert_eq!(wallet.id.len(), 36);
}

#[test]
fn update_balance() {
    let conn = establish_connection();
    let wallet = Wallet::create(&conn).unwrap();
    let balance = 100;
    let updated_wallet = Wallet::update_balance(&conn, wallet.id.as_str(), balance).unwrap();
    assert_eq!(updated_wallet.balance, balance);
}