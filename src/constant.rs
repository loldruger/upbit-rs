/// Server domain address
pub const URL_SERVER: &str = "https://api.upbit.com";

/// URL of API getting account info
pub const URL_ACCOUNTS: &str = "/v1/accounts";

/// URL of API getting order info  
pub const URL_ORDER: &str = "/v1/orders";
pub const URL_ORDER_CHANCE: &str = "/v1/orders/chance";
pub const URL_ORDER_STATUS: &str = "/v1/order";
pub const URL_ORDER_STATUS_LIST: &str = "/v1/orders";

pub const URL_ORDERBOOK: &str = "/v1/orderbook";
pub const URL_TICKER: &str = "/v1/ticker";
pub const URL_TRADES_TICKS: &str = "/v1/trades/ticks";
pub const URL_MARKET_STATE: &str = "/v1/market/all";

pub const URL_WITHDRAW: &str = "/v1/withdraw";
pub const URL_WITHDRAWS: &str = "/v1/withdraws";
pub const URL_WITHDRAWS_KRW: &str = "/v1/withdraws/krw";
pub const URL_WITHDRAWS_COIN: &str = "/v1/withdraws/coin";
pub const URL_WITHDRAWS_COIN_ADDRESS: &str = "v1/withdraws/coin_addresses";
pub const URL_WITHDRAWS_CHANCE: &str = "/v1/withdraws/chance";

/// URL of deposit API
pub const URL_DEPOSIT: &str = "/v1/deposit";
/// URL of API listing Deposit info
pub const URL_DEPOSITS: &str = "/v1/deposits";
/// URL of API inquiring generation of coin deposit address
pub const URL_DEPOSITS_GENERATE_COIN_ADDRESS: &str = "/v1/deposits/generate_coin_address";
/// URL of API getting asset you have
pub const URL_DEPOSITS_COIN_ADDRESS: &str = "/v1/deposits/coin_address";
/// URL of API listing assets you have
pub const URL_DEPOSITS_COIN_ADDRESSES: &str = "/v1/deposits/coin_addresses";
/// URL of API requesting to deposit KRW
pub const URL_DEPOSITS_KRW: &str = "/v1/deposits/krw";

/// URL of API listing candle data of minute unit
pub const URL_CANDLE_MINUTE: &str = "/v1/candles/minutes/";
pub const URL_CANDLE_DAY: &str = "/v1/candles/days";
pub const URL_CANDLE_WEEK: &str = "/v1/candles/weeks";
pub const URL_CANDLE_MONTH: &str = "/v1/candles/months";

/// Kind of order 
pub enum OrderBy {
    /// 오름차순 (Ascending)
    Asc,
    /// 내림차순 (Descending)
    Desc
}

impl ToString for OrderBy {
    fn to_string(&self) -> String {
        match self {
            OrderBy::Asc => "asc".to_owned(),
            OrderBy::Desc => "desc".to_owned(),
        }
    }
}

/// Kind of transaction type
pub enum TransactionType {
    /// 일반출금(general withdrawal)
    Default,
    /// 바로출금(instant withdrawal)
    Internal 
}

impl ToString for TransactionType {
    fn to_string(&self) -> String {
        match self {
            TransactionType::Default => "default".to_owned(),
            TransactionType::Internal => "internal".to_owned(),
        }
    }
}

/// Kind of tow factor type
pub enum TwoFactorType {
    /// 카카오페이 인증
    KakaoPay,
    /// 네이버 인증
    Naver
}

impl ToString for TwoFactorType {
    fn to_string(&self) -> String {
        match self {
            TwoFactorType::KakaoPay => "kakao_pay".to_owned(),
            TwoFactorType::Naver => "naver".to_owned(),
        }
    }
}


pub enum Currency {
    KRW,
    BTC,
    USDT,
}

pub enum CurrencyCrypto {
    BTC,
    ETH,
    ATOM
}

pub struct MarketType(Currency, CurrencyCrypto);
