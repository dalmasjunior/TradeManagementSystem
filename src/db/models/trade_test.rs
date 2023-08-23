use diesel::SqliteConnection;
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

fn create_wallet(conn: &mut SqliteConnection) -> String {
    let wallet = Wallet::create(conn).unwrap();
    wallet.id
}

fn create_user(conn: &mut SqliteConnection) -> (String, String) {
    let name = "test_user".to_string();
    let email = "test_email".to_string();
    let password = "test_password".to_string();
    let wallet_id = create_wallet(conn);

    let (user, _err) = User::create(conn, name, email, wallet_id, password);
    
    let user = user.unwrap();
    (user.id, user.wallet_id)
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
            "Arbitrum".to_string()
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
    let (user_id, wallet_id) = create_user(conn);
    let mut new_trade = gen_rand_trade(user_id, wallet_id);
    
    let trade = Trade::create(conn, &mut new_trade);
    let trade = trade.unwrap();

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
    let conn = &mut get_connection();
    let (user_id, wallet_id) = create_user(conn);

    for _ in 0..10 {
        let mut new_trade = gen_rand_trade(user_id.clone(), wallet_id.clone());
        Trade::create(conn, &mut new_trade).unwrap();
    }
    
    let _result = Trade::profit_loss(conn, "2022-01-01".to_string(), "2023-01-08".to_string(), user_id.clone(), None, None);
    assert!(_result.len() > 0);
}

#[test]
fn cumulative_fees_by_asset() {
    let conn = &mut get_connection();
    let (user_id, wallet_id) = create_user(conn);

    for _ in 0..10 {
        let mut new_trade = gen_rand_trade(user_id.clone(), wallet_id.clone());
        Trade::create(conn, &mut new_trade).unwrap();
    }
    
    let _result = Trade::profit_loss(conn, "2022-01-01".to_string(), "2023-01-08".to_string(), user_id.clone(), Some("ETH".to_string()), None);
    assert!(_result.len() > 0);
}

#[test]
fn cumulative_fees_by_trade_type() {
    let conn = &mut get_connection();
    let (user_id, wallet_id) = create_user(conn);

    for _ in 0..10 {
        let mut new_trade = gen_rand_trade(user_id.clone(), wallet_id.clone());
        Trade::create(conn, &mut new_trade).unwrap();
    }
    
    let _result = Trade::profit_loss(conn, "2022-01-01".to_string(), "2023-01-08".to_string(), user_id.clone(), None, Some("LimitBuy".to_string()));
    assert!(_result.len() > 0);
}

#[test]
fn test_profit_loss_with_asset() {
    let conn = &mut get_connection();
    let (user_id, wallet_id) = create_user(conn);
    
    let mut expected_profit_value_for_asset = 0.0;
    let mut expected_loss_value_for_asset = 0.0;

    for _ in 0..5 {
        let mut new_trade = gen_rand_trade(user_id.clone(), wallet_id.clone());
        new_trade.asset = "ETH".to_string();
        let trade = Trade::create(conn, &mut new_trade).unwrap();
        let pnl = trade.calculate_trade_pnl();
        if pnl > 0.0 {
            expected_profit_value_for_asset += pnl;
        } else {
            expected_loss_value_for_asset += pnl;
        }
    }
    
    let mut expected_profit_value_for_other_asset = 0.0;
    let mut expected_loss_value_for_other_asset = 0.0;


    for _ in 0..3 {
        let mut new_trade = gen_rand_trade(user_id.clone(), wallet_id.clone());
        new_trade.asset = "XRP".to_string();
        let trade = Trade::create(conn, &mut new_trade).unwrap();
        let pnl = trade.calculate_trade_pnl();
        if pnl > 0.0 {
            expected_profit_value_for_other_asset += pnl;
        } else {
            expected_loss_value_for_other_asset += pnl;
        }
    }
    
    let result = Trade::profit_loss(conn, "2022-01-01".to_string(), "2023-01-08".to_string(), user_id.clone(), Some("ETH".to_string()), None);
    
    assert!(!result.is_empty());

    let mut profit = 0.0;
    let mut loss = 0.0;
    for trade in result.iter() {
        profit += trade.profit;
        loss += trade.loss;
    }

    assert_eq!(profit, expected_profit_value_for_asset.round());
    assert_eq!(loss, expected_loss_value_for_asset.round());
    

    let result = Trade::profit_loss(conn, "2022-01-01".to_string(), "2023-01-08".to_string(), user_id.clone(), Some("XRP".to_string()), None);
    
    let mut profit = 0.0;
    let mut loss = 0.0;
    for trade in result.iter() {
        profit += trade.profit;
        loss += trade.loss;
    }
    // Assertion: Check if the result contains at least one entry
    assert!(!result.is_empty());

    // Example: Assert the profit and loss values for the first entry (you should adjust these values)
    assert_eq!(profit, expected_profit_value_for_other_asset.round());
    assert_eq!(loss, expected_loss_value_for_other_asset.round());

}

#[test]
fn test_profit_loss_with_tradetype() {
    let conn = &mut get_connection();
    let (user_id, wallet_id) = create_user(conn);
    
    let mut expected_profit_value_for_trade_type = 0.0;
    let mut expected_loss_value_for_trade_type = 0.0;
    
    for _ in 0..5 {
        let mut new_trade = gen_rand_trade(user_id.clone(), wallet_id.clone());
        new_trade.trade_type = "LimitBuy".to_string();
        let trade = Trade::create(conn, &mut new_trade).unwrap();
        let pnl = trade.calculate_trade_pnl();
        if pnl > 0.0 {
            expected_profit_value_for_trade_type += pnl;
        } else {
            expected_loss_value_for_trade_type += pnl;
        }
    }
    
    let result = Trade::profit_loss(conn, "2022-01-01".to_string(), "2023-01-08".to_string(), user_id.clone(), None, Some("LimitBuy".to_string()));
    
    assert!(!result.is_empty());

    let mut profit = 0.0;
    let mut loss = 0.0;
    for trade in result.iter() {
        profit += trade.profit;
        loss += trade.loss;
    }

    assert_eq!(profit, expected_profit_value_for_trade_type.round());
    assert_eq!(loss, expected_loss_value_for_trade_type.round());
}

#[test]
fn test_profit_loss_without_asset_and_tradetype() {
    let conn = &mut get_connection();
    let (user_id, wallet_id) = create_user(conn);
    
    let mut expected_profit_value = 0.0;
    let mut expected_loss_value = 0.0;
    
    for _ in 0..5 {
        let mut new_trade = gen_rand_trade(user_id.clone(), wallet_id.clone());
        
        let trade = Trade::create(conn, &mut new_trade).unwrap();
        let pnl = trade.calculate_trade_pnl();
        if pnl > 0.0 {
            expected_profit_value += pnl;
        } else {
            expected_loss_value += pnl;
        }
    }
    
    let result = Trade::profit_loss(conn, "2022-01-01".to_string(), "2023-01-08".to_string(), user_id.clone(), None, None);
    
    assert!(!result.is_empty());

    let mut profit = 0.0;
    let mut loss = 0.0;
    for trade in result.iter() {
        profit += trade.profit;
        loss += trade.loss;
    }

    assert_eq!(profit, expected_profit_value.round());
    assert_eq!(loss, expected_loss_value.round());
}

#[test]
    fn test_get_slippage_bt_dates() {
        let conn = &mut get_connection();
        let (user_id, wallet_id) = create_user(conn);
        
        let mut expected_total_slippage = 0.0;
        let mut expected_total_slippage_cost_percent = 0.0;
        let mut trades = 0;
        for _ in 0..5 {
            let mut new_trade = gen_rand_trade(user_id.clone(), wallet_id.clone());    
            let (slippage, slippage_cost_percent) = Trade::create(conn, &mut new_trade).unwrap().calculate_slippage();
            expected_total_slippage += slippage;
            expected_total_slippage_cost_percent += slippage_cost_percent;
            trades += 1;
        }        
        
        let result = Trade::get_slippage_bt_dates(conn, "2022-01-01".to_string(), "2023-01-08".to_string(), user_id.clone());
        
        let expected_average_slippage = expected_total_slippage / trades as f32;
        let expected_average_slippage_cost_percent = expected_total_slippage_cost_percent / trades as f32;

        assert_eq!(result.trader_id, user_id);
        
        assert_eq!(result.total_slippage, expected_total_slippage.round());
        assert_eq!(result.average_slippage, expected_average_slippage.round());
        assert_eq!(result.total_slippage_cost_percent, expected_total_slippage_cost_percent.round());
        assert_eq!(result.average_slippage_cost_percent, expected_average_slippage_cost_percent.round());
    }