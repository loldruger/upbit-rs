pub mod order_book;
pub mod ticker_snapshot;
pub mod trade_recent;
pub mod market_state;
pub mod candle_minute;
pub mod candle_day;
pub mod candle_month;
pub mod candle_week;

pub use order_book::OrderbookInfo;
pub use ticker_snapshot::TickerSnapshot;
pub use trade_recent::TradeRecent;
pub use market_state::MarketState;
pub use candle_minute::CandleChartMinute;
pub use candle_day::CandleChartDay;
pub use candle_week::CandleChartWeek;
pub use candle_month::CandleChartMonth;

use crate::{constant::CandleMinute, response_source::ResponseError};

pub async fn get_order_book_info(market: &str) -> Result<OrderbookInfo, ResponseError> {
    OrderbookInfo::get_orderbook_info(market).await
}

pub async fn get_ticker_snapshot(market: &str) ->Result<TickerSnapshot, ResponseError> {
    TickerSnapshot::get_ticker_snapshot(market).await
}

pub async fn get_trade_recent(market: &str, hhmmss: Option<&str>, count: i32, cursor: String, days_ago: Option<i32>) -> Result<TradeRecent, ResponseError>{
    TradeRecent::get_trade_recent(market, hhmmss, count, cursor, days_ago).await
}

pub async fn get_market_state(is_warning_shown: bool) -> Result<Vec<MarketState>, ResponseError> {
    MarketState::get_market_state(is_warning_shown).await
}

pub async fn get_candle_minute(market: &str, to: Option<String>, count: i32, candle_minute: CandleMinute) -> Result<Vec<CandleChartMinute>, ResponseError> {
    CandleChartMinute::request_candle(market, to, count, candle_minute).await
}

pub async fn get_candle_day(market: &str, count: i32, last_candle_time: Option<String>, price_unit: Option<String>) -> Result<Vec<CandleChartDay>, ResponseError> {
    CandleChartDay::request_candle(market, count, last_candle_time, price_unit).await
}

pub async fn get_candle_week(market: &str, count: i32, last_candle_time: Option<String>) -> Result<Vec<CandleChartWeek>, ResponseError> {
    CandleChartWeek::request_candle(market, count, last_candle_time).await
}

pub async fn get_candle_month(market: &str, count: i32, last_candle_time: Option<String>) -> Result<Vec<CandleChartMonth>, ResponseError> {
    CandleChartMonth::request_candle(market, count, last_candle_time).await 
}