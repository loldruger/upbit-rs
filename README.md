upbit-api on rust, with upbit api v1.3.4


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
let order_status = api_exchange::get_order_status(Some("9ca023a5-851b-4fec-9f0a-48cd83c2eaae"), None).await;
let order_status_list = api_exchange::list_order_status().await;

let order_info = api_exchange::order_by_price("KRW-ETH", OrderSide::Bid, 5000.0, 1_435_085.0, OrderType::Limit, None).await;
let order_info = api_exchange::sell_by_market_price("KRW-ETH", 1.0, "cdd92199-2897-4e14-9448-f923320408ad").await;
let order_info = api_exchange::cancel_order("cdd92199-2897-4e14-9448-f923320408ad").await;

// api_withdraw
let withdraw_info = api_withdraw::get_withdraw_info(None, Some("cdd92199-2897-4e14-9448-f923320408ad"), None).await;
let withdraw_info_list = api_withdraw::list_withdraw_info("KRW", WithdrawState::Done, None, None, 10, 0, OrderBy::Asc).await;
let withdraw_chance = api_withdraw::get_withdraw_chance("KRW", None).await;
let withdraw_result = api_withdraw::withdraw_krw(10000.0, api_withdraw::TwoFactorType::KakaoPay).await;
let withdraw_result_more_info = api_withdraw::withdraw_coin("ETH", "ETH", 0.05, "0x40268F1e99F76b658c6D52d89166EE289EfC225d", None, TransactionType::Default).await;

// api_deposit
let deposit_result = api_deposit::list_deposit_info("KRW", DepositState::Rejected, None, None, 10, 0, OrderBy::Asc).await;

// api_quotation
let order_book_info = api_quotation::get_orderbook_info("KRW-ETH").await;
let ticker_snapshot = api_quotation::get_ticker_snapshot("KRW-ETH").await;
let recent_trade_list = api_quotation::list_trade_recent("KRW-ETH", None, 3, "0".to_string(), None).await;
let market_state = api_quotation::get_market_state(true).await;

let chart_of_minute = api_quotation::get_candle_minute("KRW-ETH", None, 50, CandleMinute::Min10).await;
let chart_of_day = api_quotation::get_candle_day("KRW-ETH", 10, None, None).await;
let chart_of_week = api_quotation::get_candle_week("KRW-ETH", 10, None).await;
let chart_of_month = api_quotation::get_candle_month("KRW-ETH", 10, None).await;

```

# TroubleShooting

### 1. You must have a static ip address to get issued your own access key and secret key

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
