use crate::db::establish_connection;

fn create_wallet() -> String {
    let conn = establish_connection();
    let wallet = Wallet::create(&conn).unwrap();
    wallet.id
}

#[test]
fn create_user() {    
    let conn = establish_connection();
    let name = Some("test_user");
    let email = Some("test@email.com");
    let password = Some("test_password");
    let wallet_id = create_wallet();

    let user = User::create(&conn, name, email, wallet_id, password).unwrap();

    assert_eq!(user.name.unwrap().as_str(), name.unwrap());
    assert_eq!(user.email.unwrap().as_str(), email.unwrap());
    assert_eq!(user.wallet_id.unwrap().as_str(), wallet_id);
    assert_eq!(user.password.unwrap().as_str(), password.unwrap());
}