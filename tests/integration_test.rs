use tokio;
use upbit::{
    self,
    api_exchange::{OrderSide, OrderType},
    api_quotation::CandleMinute,
};

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

    let order_bid = upbit::api_exchange::order_by_price(
        "KRW-ETH",
        OrderSide::Bid,
        5000.0,
        1_435_085.0,
        OrderType::Limit,
        None,
    )
    .await;

    assert!(order_bid.is_ok())
}

#[tokio::test]
async fn test_order_ask_by_price() {
    upbit::set_access_key(&std::env::var("TEST_ACCESS_KEY").expect("TEST_ACCESS_KEY not set"));
    upbit::set_secret_key(&std::env::var("TEST_SECRET_KEY").expect("TEST_ACCESS_KEY not set"));

    let order_ask = upbit::api_exchange::order_by_price(
        "KRW-ETH",
        OrderSide::Ask,
        5000.0,
        10_435_085.0,
        OrderType::Limit,
        None,
    )
    .await;

    assert!(order_ask.is_ok())
}

#[tokio::test]
async fn test_get_candle_of_minute() {
    upbit::set_access_key(&std::env::var("TEST_ACCESS_KEY").expect("TEST_ACCESS_KEY not set"));
    upbit::set_secret_key(&std::env::var("TEST_SECRET_KEY").expect("TEST_ACCESS_KEY not set"));

    let candle_1 =
        upbit::api_quotation::get_candle_minute("KRW-ETH", None, 1, CandleMinute::Min1).await;

    assert!(candle_1.is_ok());

    let candle_3 =
        upbit::api_quotation::get_candle_minute("KRW-ETH", None, 1, CandleMinute::Min3).await;

    assert!(candle_3.is_ok());

    let candle_5 =
        upbit::api_quotation::get_candle_minute("KRW-ETH", None, 1, CandleMinute::Min5).await;

    assert!(candle_5.is_ok());

    let candle_10 =
        upbit::api_quotation::get_candle_minute("KRW-ETH", None, 1, CandleMinute::Min10).await;

    assert!(candle_10.is_ok());

    let candle_15 =
        upbit::api_quotation::get_candle_minute("KRW-ETH", None, 1, CandleMinute::Min15).await;

    assert!(candle_15.is_ok());

    let candle_30 =
        upbit::api_quotation::get_candle_minute("KRW-ETH", None, 1, CandleMinute::Min30).await;

    assert!(candle_30.is_ok());

    let candle_60 =
        upbit::api_quotation::get_candle_minute("KRW-ETH", None, 1, CandleMinute::Min60).await;

    assert!(candle_60.is_ok());

    // let candle_240 = upbit::api_quotation::get_candle_minute("KRW-ETH", None, 1, CandleMinute::Min240).await;

    // assert!(candle_240.is_ok());
}

#[tokio::test]
async fn test_get_candle_of_day() {
    upbit::set_access_key(&std::env::var("TEST_ACCESS_KEY").expect("TEST_ACCESS_KEY not set"));
    upbit::set_secret_key(&std::env::var("TEST_SECRET_KEY").expect("TEST_ACCESS_KEY not set"));

    let candle = upbit::api_quotation::get_candle_day("KRW-ETH", 1, None, None).await;

    assert!(candle.is_ok())
}

#[tokio::test]
async fn test_get_candle_of_week() {
    upbit::set_access_key(&std::env::var("TEST_ACCESS_KEY").expect("TEST_ACCESS_KEY not set"));
    upbit::set_secret_key(&std::env::var("TEST_SECRET_KEY").expect("TEST_ACCESS_KEY not set"));

    let candle = upbit::api_quotation::get_candle_week("KRW-ETH", 1, None).await;

    assert!(candle.is_ok())
}

#[tokio::test]
async fn test_get_candle_of_month() {
    upbit::set_access_key(&std::env::var("TEST_ACCESS_KEY").expect("TEST_ACCESS_KEY not set"));
    upbit::set_secret_key(&std::env::var("TEST_SECRET_KEY").expect("TEST_ACCESS_KEY not set"));

    let candle = upbit::api_quotation::get_candle_month("KRW-ETH", 1, None).await;

    assert!(candle.is_ok())
}

#[tokio::test]
async fn test_get_market_state() {
    upbit::set_access_key(&std::env::var("TEST_ACCESS_KEY").expect("TEST_ACCESS_KEY not set"));
    upbit::set_secret_key(&std::env::var("TEST_SECRET_KEY").expect("TEST_ACCESS_KEY not set"));

    let state = upbit::api_quotation::get_market_state(true).await;

    assert!(state.is_ok())
}
