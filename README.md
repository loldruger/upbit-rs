# 🚀 Upbit API for Rust

## 📚 사용법

### 방법 1: 환경 변수 자동 초기화 (권장)

```rust
use upbit;

#[tokio::main]
async fn main() {
    // .env 파일 또는 환경변수에서 자동으로 키를 로드
    match upbit::init() {
        Ok(()) => println!("Upbit API initialized successfully"),
        Err(e) => {
            eprintln!("Failed to initialize: {}", e);
            return;
        }
    }
    
    // 이제 API 호출 가능
    let account_info = upbit::api_exchange::get_account_info().await;
    println!("{:?}", account_info);
}
```

### 방법 2: 테스트용 초기화

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_api() {
        // 테스트용 키로 초기화
        if let Err(e) = upbit::init_for_test() {
            panic!("Failed to initialize test keys: {}", e);
        }
        
        // 테스트 코드...
    }
}
```

### 방법 3: 수동 설정

```rust
use upbit;

upbit::set_access_key("your_access_key");
upbit::set_secret_key("your_secret_key");
```

## 🛠 Requirement

이 라이브러리는 `openssl-sys` 패키지가 필요합니다.

### Linux (Ubuntu/Debian)
```bash
sudo apt update
sudo apt install libssl-dev pkg-config build-essential
```

### macOS
```bash
brew install openssl pkg-config
```

## 📦 설치

```toml
[dependencies]
upbit = "1.15.0"
```

## 🔧 설정

### 방법 1: .env 파일 사용 (권장)

1. 프로젝트 루트에 `.env` 파일을 생성하세요:

```bash
cp .env.example .env
```

2. `.env`:

```env
UPBIT_ACCESS_KEY="your_actual_access_key"
UPBIT_SECRET_KEY="your_actual_secret_key"

TEST_ACCESS_KEY="your_test_access_key"
TEST_SECRET_KEY="your_test_secret_key"
```

3. 코드에서 초기화하세요:

```rust
use upbit;

match upbit::init() {
    Ok(()) => println!("Upbit API initialized successfully"),
    Err(e) => eprintln!("Failed to initialize: {}", e),
}
```

### 방법 2: 직접 설정

```rust
use upbit;

upbit::set_access_key("your_access_key");
upbit::set_secret_key("your_secret_key");
```

### � 보안 주의사항

```rust
use upbit::*;

// api_exchange
let account_info = api_exchange::get_account_info().await;

let order_chance = api_exchange::get_order_chance("KRW-ETH").await;
// [deprecated] let order_status_list = api_exchange::get_order_status_list().await;
let order_status = api_exchange::get_order_status_by_uuid("d60dfc8a-db0a-4087-9974-fed6433eb8f1").await;
let orders_status_open = api_exchange::get_order_status_list_by_uuids("KRW-ETH", &["d60dfc8a-db0a-4087-9974-fed6433eb8f1"], OrderBy::Desc)
let order_info = api_exchange::get_order_status_list_opened("KRW-ETH", &[OrderState::Wait], 1, 10, OrderBy::Desc).await;
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
let recent_trade_list = api_quotation::get_trade_recent_list("KRW-ETH", None, 3, "0", None).await;
let market_state = api_quotation::get_market_state(true).await;

let chart_of_minute = api_quotation::get_candle_minute_list("KRW-ETH", None, 50, CandleMinute::Min10).await;
let chart_of_day = api_quotation::get_candle_day_list("KRW-ETH", 10, None, None).await;
let chart_of_week = api_quotation::get_candle_week_list("KRW-ETH", 10, None).await;
let chart_of_month = api_quotation::get_candle_month_list("KRW-ETH", 10, None).await;

```

## 🐛 TroubleShooting

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
