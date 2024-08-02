use std::any::Any;

use upbit;
use tokio;

#[tokio::test]
async fn test_get_account_info() {
    upbit::set_access_key(&std::env::var("TEST_ACCESS_KEY").expect("TEST_ACCESS_KEY not set"));
    upbit::set_secret_key(&std::env::var("TEST_SECRET_KEY").expect("TEST_ACCESS_KEY not set"));

    let account_info = upbit::api_exchange::get_account_info()
        .await
        .unwrap();

    println!("{:#?}", account_info[0]);

    match &account_info[0] {
        upbit::response::AccountsInfo {
            currency,
            balance,
            locked,
            avg_buy_price,
            avg_buy_price_modified,
            unit_currency 
        } => {
            assert!(currency.type_id() == std::any::TypeId::of::<String>());
            assert!(balance.type_id() == std::any::TypeId::of::<f64>());
            assert!(locked.type_id() == std::any::TypeId::of::<f64>());
            assert!(avg_buy_price.type_id() == std::any::TypeId::of::<f64>());
            assert!(avg_buy_price_modified.type_id() == std::any::TypeId::of::<bool>());
            assert!(unit_currency.type_id() == std::any::TypeId::of::<String>());
        }
    }
}

#[tokio::test]
async fn test_get_order_chance() {
    upbit::set_access_key(&std::env::var("TEST_ACCESS_KEY").expect("TEST_ACCESS_KEY not set"));
    upbit::set_secret_key(&std::env::var("TEST_SECRET_KEY").expect("TEST_ACCESS_KEY not set"));

    let order_chance = upbit::api_exchange::get_order_chance("KRW-BTC")
        .await
        .unwrap();

    match &order_chance {
        upbit::response::OrderChance {
            bid_fee,
            ask_fee,
            market,
            ask_types,
            bid_types,
            bid_account,
            ask_account
        } => {
            assert!(bid_fee.type_id() == std::any::TypeId::of::<f32>());
            assert!(ask_fee.type_id() == std::any::TypeId::of::<f32>());
            assert!(market.type_id() == std::any::TypeId::of::<upbit::response::ObjectMarket>());
            assert!(market.id.type_id() == std::any::TypeId::of::<String>());
            assert!(market.name.type_id() == std::any::TypeId::of::<String>());
            assert!(market.order_sides.type_id() == std::any::TypeId::of::<Vec<upbit::api_exchange::OrderSide>>());
            assert!(market.bid.type_id() == std::any::TypeId::of::<upbit::response::ObjectAskBid>());
            assert!(market.bid.currency.type_id() == std::any::TypeId::of::<String>());
            assert!(market.bid.price_unit.type_id() == std::any::TypeId::of::<Option<String>>());
            assert!(market.bid.min_total.type_id() == std::any::TypeId::of::<u32>());
            assert!(market.ask.type_id() == std::any::TypeId::of::<upbit::response::ObjectAskBid>());
            assert!(market.ask.currency.type_id() == std::any::TypeId::of::<String>());
            assert!(market.ask.price_unit.type_id() == std::any::TypeId::of::<Option<String>>());
            assert!(market.ask.min_total.type_id() == std::any::TypeId::of::<u32>());
            assert!(market.max_total.type_id() == std::any::TypeId::of::<u64>());
            assert!(market.state.type_id() == std::any::TypeId::of::<String>());
            assert!(ask_types.type_id() == std::any::TypeId::of::<Option<Vec<String>>>());
            assert!(bid_types.type_id() == std::any::TypeId::of::<Option<Vec<String>>>());
            assert!(bid_account.type_id() == std::any::TypeId::of::<upbit::response::AccountsInfo>());
            assert!(bid_account.currency.type_id() == std::any::TypeId::of::<String>());
            assert!(bid_account.balance.type_id() == std::any::TypeId::of::<f64>());
            assert!(bid_account.locked.type_id() == std::any::TypeId::of::<f64>());
            assert!(bid_account.avg_buy_price.type_id() == std::any::TypeId::of::<f64>());
            assert!(bid_account.avg_buy_price_modified.type_id() == std::any::TypeId::of::<bool>());
            assert!(bid_account.unit_currency.type_id() == std::any::TypeId::of::<String>());
            assert!(ask_account.type_id() == std::any::TypeId::of::<upbit::response::AccountsInfo>());
            assert!(ask_account.currency.type_id() == std::any::TypeId::of::<String>());
            assert!(ask_account.balance.type_id() == std::any::TypeId::of::<f64>());
            assert!(ask_account.locked.type_id() == std::any::TypeId::of::<f64>());
            assert!(ask_account.avg_buy_price.type_id() == std::any::TypeId::of::<f64>());
            assert!(ask_account.avg_buy_price_modified.type_id() == std::any::TypeId::of::<bool>());
            assert!(ask_account.unit_currency.type_id() == std::any::TypeId::of::<String>());
        }
    }
}