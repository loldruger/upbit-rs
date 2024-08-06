use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// Server domain address
pub const URL_SERVER: &str = "https://api.upbit.com";

/// URL of API getting account info
pub const URL_ACCOUNTS: &str = "/v1/accounts";

/// URL of API getting order info  
pub const URL_ORDER: &str = "/v1/orders";
/// URL of API getting order chance
pub const URL_ORDER_CHANCE: &str = "/v1/orders/chance";
/// URL of API getting order status
pub const URL_ORDER_STATUS: &str = "/v1/order";
/// URL of API getting order status list
#[deprecated(since = "1.6.0")]
pub const URL_ORDER_STATUS_LIST: &str = "/v1/orders";
/// URL of API getting order status by uuids
pub const URL_ORDER_STATUS_BY_UUID: &str = "/v1/orders/uuids";
/// URL of API getting order status waiting for trades
pub const URL_ORDER_STATUS_OPEN: &str = "/v1/orders/open";
/// URL of API getting order status closed
pub const URL_ORDER_STATUS_CLOSED: &str = "/v1/orders/closed";

/// URL of API getting order book
pub const URL_ORDERBOOK: &str = "/v1/orderbook";
/// URL of API getting ticker
pub const URL_TICKER: &str = "/v1/ticker";
/// URL of API getting trandes ticks
pub const URL_TRADES_TICKS: &str = "/v1/trades/ticks";
/// URL of API getting market state
pub const URL_MARKET_STATE: &str = "/v1/market/all";

/// URL of API getting withdraw info
pub const URL_WITHDRAW: &str = "/v1/withdraw";
/// URL of API getting withdraw info list
pub const URL_WITHDRAWS: &str = "/v1/withdraws";
/// URL of API withdrawing KRW
pub const URL_WITHDRAWS_KRW: &str = "/v1/withdraws/krw";
/// URL of API withdrawing coin
pub const URL_WITHDRAWS_COIN: &str = "/v1/withdraws/coin";
/// URL of API getting coin address
pub const URL_WITHDRAWS_COIN_ADDRESS: &str = "v1/withdraws/coin_addresses";
/// URL of API getting withdraw chance
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
/// URL of API listing candle data of day unit
pub const URL_CANDLE_DAY: &str = "/v1/candles/days";
/// URL of API listing candle data of week unit
pub const URL_CANDLE_WEEK: &str = "/v1/candles/weeks";
/// URL of API listing candle data of month unit
pub const URL_CANDLE_MONTH: &str = "/v1/candles/months";

/// Kind of order 
pub enum OrderBy {
    /// 오름차순 (Ascending)
    Asc,
    /// 내림차순 (Descending)
    Desc
}

impl Display for OrderBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderBy::Asc => write!(f, "asc"),
            OrderBy::Desc => write!(f, "desc"),
        }
    }
}

// impl ToString for OrderBy {
//     fn to_string(&self) -> String {
//         match self {
//             OrderBy::Asc => "asc".to_owned(),
//             OrderBy::Desc => "desc".to_owned(),
//         }
//     }
// }

impl From<&str> for OrderBy {
    fn from(value: &str) -> Self {
        match value {
            "asc" => Self::Asc,
            "desc" => Self::Desc,
            _ => panic!()
        }
    }
}

/// Kind of transaction type
#[derive(Debug)]
pub enum TransactionType {
    /// 일반 입출금(general withdrawal or deposit)
    Default,
    /// 바로 입출금(instant withdrawal or deposit)
    Internal 
}

impl Display for TransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionType::Default => write!(f, "default"),
            TransactionType::Internal => write!(f, "internal"),
        }
    }
}

// impl ToString for WithdrawType {
//     fn to_string(&self) -> String {
//         match self {
//             WithdrawType::Default => "default".to_owned(),
//             WithdrawType::Internal => "internal".to_owned(),
//         }
//     }
// }

impl From<&str> for TransactionType {
    fn from(value: &str) -> Self {
        match value {
            "default" => Self::Default,
            "internal" => Self::Internal,
            _ => panic!("")
        }
    }
}

/// Kind of tow factor type
pub enum TwoFactorType {
    /// 카카오페이 인증
    #[deprecated(since = "1.7.3", note = "Use Kakao instead")]
    KakaoPay,
    /// 네이버 인증
    Naver,
    KaKao
}

impl Display for TwoFactorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TwoFactorType::KakaoPay => write!(f, "kakao_pay"),
            TwoFactorType::Naver => write!(f, "naver"),
            TwoFactorType::KaKao => write!(f, "kakao"),
        }
    }
}

// impl ToString for TwoFactorType {
//     fn to_string(&self) -> String {
//         match self {
//             TwoFactorType::KakaoPay => "kakao_pay".to_owned(),
//             TwoFactorType::Naver => "naver".to_owned(),
//         }
//     }
// }

/// List of transaction type
#[derive(Debug)]
pub enum TransferType {
    /// 출금
    Withdraw,
    /// 입금
    Deposit
}

impl Display for TransferType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransferType::Withdraw => write!(f, "withdraw"),
            TransferType::Deposit => write!(f, "deposit"),
        }
    }
}

// impl ToString for TransactionType {
//     fn to_string(&self) -> String {
//         match self {
//             TransactionType::Withdraw => "withdraw".to_owned(),
//             TransactionType::Deposit => "deposit".to_owned(),
//         }
//     }
// }

impl From<&str> for TransferType {
    fn from(value: &str) -> Self {
        match value {
            "withdraw" => Self::Withdraw,
            "deposit" => Self::Deposit,
            _ => panic!("")
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AskType {
    BestFOK,
    BestIOC,
    Limit,
    LimitFOK,
    LimitIOC,
    Market
}

impl Display for AskType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AskType::BestFOK => write!(f, "best_fok"),
            AskType::BestIOC => write!(f, "best_ioc"),
            AskType::Limit => write!(f, "limit"),
            AskType::LimitFOK => write!(f, "limit_fok"),
            AskType::LimitIOC => write!(f, "limit_ioc"),
            AskType::Market => write!(f, "market"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum BidType {
    BestFOK,
    BestIOC,
    Limit,
    LimitFOK,
    LimitIOC,
    Price
}

impl Display for BidType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BidType::BestFOK => write!(f, "best_fok"),
            BidType::BestIOC => write!(f, "best_ioc"),
            BidType::Limit => write!(f, "limit"),
            BidType::LimitFOK => write!(f, "limit_fok"),
            BidType::LimitIOC => write!(f, "limit_ioc"),
            BidType::Price => write!(f, "price"),
        }
    }
}

impl From<&str> for AskType {
    fn from(value: &str) -> Self {
        match value {
            "best_fok" => Self::BestFOK,
            "best_ioc" => Self::BestIOC,
            "limit" => Self::Limit,
            "limit_fok" => Self::LimitFOK,
            "limit_ioc" => Self::LimitIOC,
            "market" => Self::Market,
            _ => panic!("")
        }
    }
}

impl From<&str> for BidType {
    fn from(value: &str) -> Self {
        match value {
            "best_fok" => Self::BestFOK,
            "best_ioc" => Self::BestIOC,
            "limit" => Self::Limit,
            "limit_fok" => Self::LimitFOK,
            "limit_ioc" => Self::LimitIOC,
            "price" => Self::Price,
            _ => panic!("")
        }
    }
}

// pub enum Currency {
//     KRW,
//     BTC,
//     USDT,
// }

// pub enum CurrencyCrypto {
//     BTC,
//     ETH,
//     ATOM
// }

// pub struct MarketType(Currency, CurrencyCrypto);
