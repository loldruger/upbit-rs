pub const URL_SERVER: &str = "https://api.upbit.com";

pub const URL_ACCOUNTS: &str = "/v1/accounts";
pub const URL_ORDER_CHANCE: &str = "/v1/orders/chance";
pub const URL_ORDER_STATUS: &str = "/v1/order";
pub const URL_ORDER_STATUS_LIST: &str = "/v1/orders";
pub const URL_ORDER: &str = "/v1/orders";

pub const URL_ORDERBOOK: &str = "/v1/orderbook";
pub const URL_TICKER: &str = "/v1/ticker";
pub const URL_TRADES_TICKS: &str = "/v1/trades/ticks";
pub const URL_MARKET_STATE: &str = "/v1/market/all";

pub const URL_WITHDRAW: &str = "/v1/withdraw";
pub const URL_WITHDRAWS: &str = "/v1/withdraws";

// pub const URL_CANDLE_MINUTE: &str = "/v1/candles/minutes/";
// pub const URL_CANDLE_DAY: &str = "/v1/candles/days";
// pub const URL_CANDLE_WEEK: &str = "/v1/candles/weeks";
// pub const URL_CANDLE_MONTH: &str = "/v1/candles/months";

pub enum OrderBy {
    Asc,
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
