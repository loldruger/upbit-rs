upbit-api on rust, with upbit api v1.5.0


this crate is dependant on `tokio`

it also requires `openssl-sys` package

# Set access key and secret key
```rust
use upbit::*;

upbit::set_access_key("");
upbit::set_secret_key("");
```

# APIs
```rust
use upbit::*;

// api_exchange
let account_info = api_exchange::get_account_info().await;

let order_chance = api_exchange::get_order_chance("KRW-ETH").await;
// [deprecated] let order_status_list = api_exchange::get_order_status_list().await;
let order_status = api_exchange::get_order_status_by_uuid("d60dfc8a-db0a-4087-9974-fed6433eb8f1").await;
let orders_status_open = api_exchange::get_order_status_list_by_uuids("KRW-ETH", &["d60dfc8a-db0a-4087-9974-fed6433eb8f1"], OrderBy::Desc)
let order_info = upbit::api_exchange::get_order_status_list_opened("KRW-ETH", &[OrderState::Wait], 1, 10, OrderBy::Desc).await;
let orders_status_closed = api_exchange::get_order_status_list_closed("KRW-ETH", &[OrderState::Done], None, None, 10, OrderBy::Desc).await;

let order_bid = api_exchange::order_by_price("KRW-ETH", OrderSide::Bid, 5000.0, 1_435_085.0, OrderType::Limit, None).await;
let order_ask = api_exchange::order_by_price("KRW-ETH", OrderSide::Ask, 5000.0, 10_435_085.0, OrderType::Limit, None).await;

let order_info = api_exchange::cancel_order("cdd92199-2897-4e14-9448-f923320408ad").await;

// api_withdraw
let withdraw_result = api_withdraw::withdraw_krw(10000.0, api_withdraw::TwoFactorType::Kaka).await;
let withdraw_info = api_withdraw::get_withdraw_info(None, Some("cdd92199-2897-4e14-9448-f923320408ad"), None).await;
let withdraw_info_list = api_withdraw::get_witrhdraw_info_list("KRW", WithdrawState::Done, None, None, 10, 0, OrderBy::Asc).await;
let withdraw_chance = api_withdraw::get_withdraw_chance("KRW", None).await;
let withdraw_result_more_info = api_withdraw::withdraw_coin("ETH", "ETH", 0.05, "0x40268F1e99F76b658c6D52d89166EE289EfC225d", None, TransactionType::Default).await;

// api_deposit
let deposit_result = api_deposit::deposit_krw(10000.0, api_withdraw::TwoFactorType::Kaka).await
let deposit_result = api_deposit::get_deposit_info(Some("KRW"), None, None).await;
let deposit_result = api_deposit::get_deposit_info_list("KRW", DepositState::Rejected, None, None, 10, 0, OrderBy::Asc).await;

let coin_address_info = api_deposit::get_coin_address_info("ETH", "ETH").await:
let coin_address_info_list = api_deposit::get_coin_address_info_list().await;

// api_quotation
let order_book_info = api_quotation::get_orderbook_info("KRW-ETH").await;
let ticker_snapshot = api_quotation::get_ticker_snapshot("KRW-ETH").await;
let recent_trade_list = api_quotation::get_trade_recent_list("KRW-ETH", None, 3, "0".to_string(), None).await;
let market_state = api_quotation::get_market_state(true).await;

let chart_of_minute = api_quotation::get_candle_minute_list("KRW-ETH", None, 50, CandleMinute::Min10).await;
let chart_of_day = api_quotation::get_candle_day_list("KRW-ETH", 10, None, None).await;
let chart_of_week = api_quotation::get_candle_week_list("KRW-ETH", 10, None).await;
let chart_of_month = api_quotation::get_candle_month_list("KRW-ETH", 10, None).await;

```

# TroubleShooting

### 1. You must have a static ip address to issue your own access key and secret key

### 2. Failed to run custom build command for `openssl-sys vX.X.XX`

If you have trouble installing with this error: failed to run custom build command for `openssl-sys vX.X.XX`, 

try
```
macOS
$ brew install openssl@1.1

Arch Linux
$ sudo pacman -S pkg-config openssl

Debian and Ubuntu
$ sudo apt-get install pkg-config libssl-dev

Fedora
$ sudo dnf install pkg-config openssl-devel
```
referenced from https://github.com/sfackler/rust-openssl/issues/855#issuecomment-450057552
