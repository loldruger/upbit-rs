use upbit::{self, api_exchange::{OrderSide, OrderType}};
use tokio;

#[tokio::test]
async fn test_get_account_info() {
    upbit::set_access_key(&std::env::var("TEST_ACCESS_KEY").expect("TEST_ACCESS_KEY not set"));
    upbit::set_secret_key(&std::env::var("TEST_SECRET_KEY").expect("TEST_ACCESS_KEY not set"));

    let account_info = upbit::api_exchange::get_account_info().await;

    assert!(account_info.is_ok())
}

#[tokio::test]
async fn test_get_order_chance() {
    upbit::set_access_key(&std::env::var("TEST_ACCESS_KEY").expect("TEST_ACCESS_KEY not set"));
    upbit::set_secret_key(&std::env::var("TEST_SECRET_KEY").expect("TEST_ACCESS_KEY not set"));

    let order_chance = upbit::api_exchange::get_order_chance("KRW-ETH").await;

    assert!(order_chance.is_ok())
}

#[tokio::test]
async fn test_order_bid_by_price() {
    upbit::set_access_key(&std::env::var("TEST_ACCESS_KEY").expect("TEST_ACCESS_KEY not set"));
    upbit::set_secret_key(&std::env::var("TEST_SECRET_KEY").expect("TEST_ACCESS_KEY not set"));

    let order_bid = upbit::api_exchange::order_by_price("KRW-ETH", OrderSide::Bid, 5000.0, 1_435_085.0, OrderType::Limit, None).await;

    assert!(order_bid.is_ok())
}

#[tokio::test]
async fn test_order_ask_by_price() {
    upbit::set_access_key(&std::env::var("TEST_ACCESS_KEY").expect("TEST_ACCESS_KEY not set"));
    upbit::set_secret_key(&std::env::var("TEST_SECRET_KEY").expect("TEST_ACCESS_KEY not set"));

    let order_ask = upbit::api_exchange::order_by_price("KRW-ETH", OrderSide::Ask, 5000.0, 10_435_085.0, OrderType::Limit, None).await;

    assert!(order_ask.is_ok())
}