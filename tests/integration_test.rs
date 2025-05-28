use tokio;
use upbit::{
    self,
    api_deposit::DepositState,
    api_exchange::{OrderSide, OrderState, OrderType},
    api_quotation::CandleMinute,
    api_withdraw::WithdrawState,
    constant::{OrderBy, TransactionType},
};

/// 테스트용 공통 설정 함수
fn setup_test_keys() {
    // .env 파일에서 자동으로 키를 로드
    if let Err(e) = upbit::init_for_test() {
        panic!(
            "Failed to initialize test keys: {}. Please check your .env file.",
            e
        );
    }
}

#[tokio::test]
async fn test_get_account_info() {
    setup_test_keys();

    let account_info = upbit::api_exchange::get_account_info().await;

    assert!(account_info.is_ok())
}

#[tokio::test]
async fn test_get_order_chance() {
    setup_test_keys();

    let order_chance = upbit::api_exchange::get_order_chance("KRW-ETH").await;

    assert!(order_chance.is_ok())
}

#[tokio::test]
async fn test_order_bid_by_price() {
    setup_test_keys();

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
    setup_test_keys();

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
async fn test_get_order_status_by_uuid() {
    setup_test_keys();

    let order_status =
        upbit::api_exchange::get_order_status_by_uuid("d60dfc8a-db0a-4087-9974-fed6433eb8f1").await;

    assert!(order_status.is_ok())
}

#[tokio::test]
async fn test_get_order_status_by_uuids() {
    setup_test_keys();

    let order_status = upbit::api_exchange::get_order_status_list_by_uuids(
        "KRW-ETH",
        &["d60dfc8a-db0a-4087-9974-fed6433eb8f1"],
        OrderBy::Desc,
    )
    .await;

    assert!(order_status.is_ok())
}

#[tokio::test]
async fn test_get_order_status_opened() {
    setup_test_keys();

    let order_open = upbit::api_exchange::get_order_status_list_opened(
        "KRW-ETH",
        &[OrderState::Wait],
        1,
        10,
        OrderBy::Desc,
    )
    .await;

    assert!(order_open.is_ok())
}

#[tokio::test]
async fn test_get_order_status_closed() {
    setup_test_keys();

    let order_closed = upbit::api_exchange::get_order_status_list_closed(
        "KRW-ETH",
        &[OrderState::Done],
        None,
        None,
        10,
        OrderBy::Desc,
    )
    .await;

    assert!(order_closed.is_ok())
}

#[tokio::test]
async fn test_get_candle_of_minute() {
    setup_test_keys();

    let candle_1 =
        upbit::api_quotation::get_candle_minute_list("KRW-ETH", None, 1, CandleMinute::Min1).await;

    assert!(candle_1.is_ok());

    let candle_3 =
        upbit::api_quotation::get_candle_minute_list("KRW-ETH", None, 1, CandleMinute::Min3).await;

    assert!(candle_3.is_ok());

    let candle_5 =
        upbit::api_quotation::get_candle_minute_list("KRW-ETH", None, 1, CandleMinute::Min5).await;

    assert!(candle_5.is_ok());

    let candle_10 =
        upbit::api_quotation::get_candle_minute_list("KRW-ETH", None, 1, CandleMinute::Min10).await;

    assert!(candle_10.is_ok());

    let candle_15 =
        upbit::api_quotation::get_candle_minute_list("KRW-ETH", None, 1, CandleMinute::Min15).await;

    assert!(candle_15.is_ok());

    let candle_30 =
        upbit::api_quotation::get_candle_minute_list("KRW-ETH", None, 1, CandleMinute::Min30).await;

    assert!(candle_30.is_ok());

    let candle_60 =
        upbit::api_quotation::get_candle_minute_list("KRW-ETH", None, 1, CandleMinute::Min60).await;

    assert!(candle_60.is_ok());

    // let candle_240 = upbit::api_quotation::get_candle_minute("KRW-ETH", None, 1, CandleMinute::Min240).await;

    // assert!(candle_240.is_ok());
}

#[tokio::test]
async fn test_get_candle_of_day() {
    setup_test_keys();

    let candle = upbit::api_quotation::get_candle_day_list("KRW-ETH", 1, None, None).await;

    assert!(candle.is_ok())
}

#[tokio::test]
async fn test_get_candle_of_week() {
    setup_test_keys();

    let candle = upbit::api_quotation::get_candle_week_list("KRW-ETH", 1, None).await;

    assert!(candle.is_ok())
}

#[tokio::test]
async fn test_get_candle_of_month() {
    setup_test_keys();

    let candle = upbit::api_quotation::get_candle_month_list("KRW-ETH", 1, None).await;

    assert!(candle.is_ok())
}

#[tokio::test]
async fn test_get_market_state() {
    setup_test_keys();

    let state = upbit::api_quotation::get_market_state_list(true).await;

    assert!(state.is_ok())
}

#[tokio::test]
async fn test_get_ticker_snapshot() {
    setup_test_keys();

    let list = upbit::api_quotation::get_ticker_snapshot_list(&["KRW-ETH"]).await;

    assert!(list.is_ok());

    let list_bunch = upbit::api_quotation::get_ticker_snapshot_list(&["KRW-BTC", "KRW-ETH"]).await;

    assert!(list_bunch.is_ok());
}

#[tokio::test]
async fn test_get_trade_recent_list() {
    setup_test_keys();

    let list =
        upbit::api_quotation::get_trade_recent_list("KRW-ETH", Some("120101"), 1, "0", None).await;

    assert!(list.is_ok());
}

#[tokio::test]
async fn test_get_order_book() {
    setup_test_keys();

    let order_book = upbit::api_quotation::get_order_book_info_list(&["KRW-ETH"]).await;

    assert!(order_book.is_ok());
}

#[tokio::test]
async fn test_get_withdraw_address() {
    setup_test_keys();

    let address = upbit::api_withdraw::get_withdraw_address_list().await;

    assert!(address.is_ok());
}

#[tokio::test]
async fn test_get_withdraw_chance() {
    setup_test_keys();

    let chance = upbit::api_withdraw::get_withdraw_chance("ETH", "ETH").await;

    assert!(chance.is_ok());
}

#[tokio::test]
async fn test_withdraw_coin() {
    setup_test_keys();

    let info = upbit::api_withdraw::withdraw_coin(
        "ETH",
        "ETH",
        0.02,
        "0x40268F1e99F76b658c6D52d89166EE289EfC225d",
        None,
        TransactionType::Default,
    )
    .await;

    assert!(info.is_ok());
}

#[tokio::test]
async fn test_get_withdraw_info() {
    setup_test_keys();

    let info = upbit::api_withdraw::get_withdraw_info(Some("KRW"), None, None).await;

    assert!(info.is_ok());

    let uuid = info.unwrap().uuid;

    let info = upbit::api_withdraw::get_withdraw_info(None, Some(&uuid), None).await;

    assert!(info.is_ok());

    let txid = info.unwrap().txid;

    let info = upbit::api_withdraw::get_withdraw_info(None, None, Some(&txid)).await;

    assert!(info.is_ok());
}

#[tokio::test]
async fn test_get_withdraw_list() {
    setup_test_keys();

    let infos = upbit::api_withdraw::get_withdraw_info_list(
        "KRW",
        WithdrawState::Done,
        None,
        None,
        10,
        0,
        OrderBy::Asc,
    )
    .await;

    assert!(infos.is_ok());
}

#[tokio::test]
async fn test_get_deposit_info() {
    setup_test_keys();

    let info = upbit::api_deposit::get_deposit_info_by_currency("KRW").await;

    assert!(info.is_ok());
}

#[tokio::test]
async fn test_get_deposit_list() {
    setup_test_keys();

    let infos = upbit::api_deposit::get_deposit_info_list(
        "KRW",
        DepositState::Accepted,
        None,
        None,
        10,
        0,
        OrderBy::Asc,
    )
    .await;

    assert!(infos.is_ok());
}

#[tokio::test]
async fn test_get_coin_address_info() {
    setup_test_keys();

    let info = upbit::api_deposit::get_coin_address_info("ETH", "ETH").await;

    assert!(info.is_ok());
}

#[tokio::test]
async fn test_get_coin_address_list() {
    setup_test_keys();

    let infos = upbit::api_deposit::get_coin_address_info_list().await;

    assert!(infos.is_ok());
}
