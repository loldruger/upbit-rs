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

pub const URL_WITHDRAWS: &str = "/v1/withdraws";

pub enum CandleMinute {
    Min1,
    Min3,
    Min5,
    Min10,
    Min15,
    Min30,
    Min60,
    Min240
}

impl From<CandleMinute> for u8 {
    fn from(value: CandleMinute) -> Self {
        match value {
            CandleMinute::Min1 => 1,
            CandleMinute::Min3 => 3,
            CandleMinute::Min5 => 5,
            CandleMinute::Min10 => 10,
            CandleMinute::Min15 => 15,
            CandleMinute::Min30 => 30,
            CandleMinute::Min60 => 60,
            CandleMinute::Min240 => 240,
        }
    }
}

pub enum UrlAssociates {
    UrlCandleMinute(CandleMinute),
    UrlCandleWeek,
    UrlCandleDay,
    UrlCandleMonth
}

impl From<UrlAssociates> for String {
    fn from(value: UrlAssociates) -> Self {
        match value {
            UrlAssociates::UrlCandleMinute(minute) => format!("/v1/candles/minutes/{}", Into::<u8>::into(minute)),
            UrlAssociates::UrlCandleWeek => "/v1/candles/weeks".to_owned(),
            UrlAssociates::UrlCandleDay => "/v1/candles/days".to_owned(),
            UrlAssociates::UrlCandleMonth => "/v1/candles/months".to_owned(),
        }
    }
}

// pub const URL_CANDLE_MINUTE: &str = "/v1/candles/minutes/";
// pub const URL_CANDLE_DAY: &str = "/v1/candles/days";
// pub const URL_CANDLE_WEEK: &str = "/v1/candles/weeks";
// pub const URL_CANDLE_MONTH: &str = "/v1/candles/months";

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OrderSide {
    BID,
    ASK,
}

impl From<OrderSide> for &str {
    fn from(order_side: OrderSide) -> &'static str {
        match order_side {
            OrderSide::BID => "bid",
            OrderSide::ASK => "ask",
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OrderType {
    LIMIT,
    PRICE,
    MARKET,
}

impl From<OrderType> for &str {
    fn from(order_type: OrderType) -> &'static str {
        match order_type {
            OrderType::LIMIT => "limit",
            OrderType::PRICE => "price",
            OrderType::MARKET => "market",
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
