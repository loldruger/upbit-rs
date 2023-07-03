upbit-api on rust, with upbit api v1.3.4


this crate is dependant on `sqlx`, `tokio`, `postgres`

it also requires `openssl-sys` package

# Set access key and secret key
```rust
use rust_upbit_api::*;

rust_upbit_api::set_access_key("");
rust_upbit_api::set_secret_key("");
```

# Apis
```rust
use rust_upbit_api::*;

let order_info = api::order_by_price("KRW-ETH", OrdSide::BID, 5000.0, 1_435_085.0, OrdType::LIMIT, None).await.unwrap();
let order_info = api::sell_by_market_price("KRW-ETH", 1.0, "cdd92199-2897-4e14-9448-f923320408ad").await;
let order_info = api::cancel_order("cdd92199-2897-4e14-9448-f923320408ad").await;

let account_info = rust_upbit_api::api::get_account_info().await;
let order_state = api::get_order_state(None, None).await.unwrap();
let order_state_list = api::get_order_state_list().await.unwrap();

let order_book_info = api::OrderbookInfo::get_orderbook_info("KRW-ETH").await;
let asdf = api::TickerSnapshot::request("KRW-ETH").await;
let asdf = api::TradeRecent::request("KRW-ETH", None, 3, "0".to_string(), None).await;
let asdf = api::MarketState::request(true).await;

let chart_by_minute = api::CandleChartMinute::request_candle("KRW-ETH", None, 50, CandleMinute::Min10).await.unwrap();
let chart_by_day = api::CandleChartDay::request_candle("KRW-ETH", 10, None, None).await;
let chart_by_week = api::CandleChartWeek::request_candle("KRW-ETH", 10, None).await;
let chart_by_month = api::CandleChartMonth::request_candle("KRW-ETH", 10, None).await;

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
