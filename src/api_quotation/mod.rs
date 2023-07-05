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

use crate::{constant::CandleMinute, response::ResponseError};

/// 호가 정보를 조회한다. (Inquiry bid price and offered price.)
/// 
/// # Example
/// ```rust
/// let order_book_info = api_quotation::get_order_book_info("KRW-ETH").await;
/// ```
/// # Response
///  * orderbook_unit 리스트에는 15호가 정보가 들어가며 차례대로 1호가, 2호가 ... 15호가의 정보를 담고 있습니다.
///  * orderbook_unit list contains information of 15 quotes of bid/ask price, in order, 1st, 2nd .. 15th quote 
/// 
/// ```json
/// [
///  {
///    "market": "KRW-BTC",
///    "timestamp": 1529910247984,
///    "total_ask_size": 8.83621228,
///    "total_bid_size": 2.43976741,
///    "orderbook_units": [
///      {
///        "ask_price": 6956000,
///        "bid_price": 6954000,
///        "ask_size": 0.24078656,
///        "bid_size": 0.00718341
///      },
///      {
///        "ask_price": 6958000,
///        "bid_price": 6953000,
///        "ask_size": 1.12919,
///        "bid_size": 0.11500074
///      },
///      {
///        "ask_price": 6960000,
///        "bid_price": 6952000,
///        "ask_size": 0.08614137,
///        "bid_size": 0.19019028
///      },
///      {
///        "ask_price": 6962000,
///        "bid_price": 6950000,
///        "ask_size": 0.0837203,
///        "bid_size": 0.28201649
///      },
///      {
///        "ask_price": 6964000,
///        "bid_price": 6949000,
///        "ask_size": 0.501885,
///        "bid_size": 0.01822085
///      },
///      {
///        "ask_price": 6965000,
///        "bid_price": 6946000,
///        "ask_size": 1.12517189,
///        "bid_size": 0.0002
///      },
///      {
///        "ask_price": 6968000,
///        "bid_price": 6945000,
///        "ask_size": 2.89900477,
///        "bid_size": 0.03597913
///      },
///      {
///        "ask_price": 6970000,
///        "bid_price": 6944000,
///        "ask_size": 0.2044231,
///        "bid_size": 0.39291445
///      },
///      {
///        "ask_price": 6972000,
///        "bid_price": 6939000,
///        "ask_size": 2.55280097,
///        "bid_size": 0.12963816
///      },
///      {
///        "ask_price": 6974000,
///        "bid_price": 6937000,
///        "ask_size": 0.01308832,
///        "bid_size": 1.2684239
///      }
///    ]
///  }
/// ]
/// ```
/// | field             | description                   | type         |
/// |:------------------|:------------------------------|:-------------|
/// | market | 마켓 코드 | String |
/// | timestamp | 호가 생성 시각 | Long |
/// | total_ask_size | 호가 매도 총 잔량 | Double |
/// | total_bid_size | 호가 매수 총 잔량 | Double |
/// | orderbook_units | 호가 | List of Objects |
/// | ask_price | 매도호가 | Double |
/// | bid_price | 매수호가 | Double |
/// | ask_size | 매도 잔량 | Double |
/// | bid_size | 매수 잔량 | Double |
pub async fn get_order_book_info(market: &str) -> Result<OrderbookInfo, ResponseError> {
    OrderbookInfo::get_orderbook_info(market).await
}

/// 요청 당시 종목의 스냅샷을 반환한다. (Return the snapshot of the ticker at the moment of query.)
/// 
/// # Example
/// ```rust
/// let ticker_snapshot = api_quotation::get_ticker_snapshot("KRW-ETH").await;
/// ```
/// # Response
/// * 아래 응답의 `change`, `change_price`, `change_rate`, `signed_change_price`, `signed_change_rate` 필드들은 전일종가 대비 값입니다.
/// * The fields `change`, `change_price`, `change_rate`, `signed_change_price`, and `signed_change_rate` in the response below are values compared to the previous day’s closing price.
/// ```json
/// [
///   {
///     "market": "KRW-BTC",
///     "trade_date": "20180418",
///     "trade_time": "102340",
///     "trade_date_kst": "20180418",
///     "trade_time_kst": "192340",
///     "trade_timestamp": 1524047020000,
///     "opening_price": 8450000,
///     "high_price": 8679000,
///     "low_price": 8445000,
///     "trade_price": 8621000,
///     "prev_closing_price": 8450000,
///     "change": "RISE",
///     "change_price": 171000,
///     "change_rate": 0.0202366864,
///     "signed_change_price": 171000,
///     "signed_change_rate": 0.0202366864,
///     "trade_volume": 0.02467802,
///     "acc_trade_price": 108024804862.58253,
///     "acc_trade_price_24h": 232702901371.09308,
///     "acc_trade_volume": 12603.53386105,
///     "acc_trade_volume_24h": 27181.31137002,
///     "highest_52_week_price": 28885000,
///     "highest_52_week_date": "2018-01-06",
///     "lowest_52_week_price": 4175000,
///     "lowest_52_week_date": "2017-09-25",
///     "timestamp": 1524047026072
///   }
/// ]
/// ```
/// | field             | description                   | type         |
/// |:------------------|:------------------------------|:-------------|
/// | market | 종목 구분 코드 | String |
/// | trade_date | 최근 거래 일자(UTC) <br> 포맷: yyyyMMdd | String |
/// | trade_time | 최근 거래 시각(UTC) <br> 포맷: HHmmss | String |
/// | trade_date_kst | 최근 거래 일자(KST) <br> 포맷: yyyyMMdd | String |
/// | trade_time_kst | 최근 거래 시각(KST) <br> 포맷: HHmmss | String |
/// | trade_timestamp | 최근 거래 일시(UTC) <br> 포맷: Unix Timestamp | Long |
/// | opening_price | 시가 | Double |
/// | high_price | 고가 | Double |
/// | low_price | 저가 | Double |
/// | trade_price | 종가(현재가) | Double |
/// | prev_closing_price | 전일 종가(UTC 0시 기준) | Double |
/// | change | EVEN : 보합 <br> RISE : 상승 <br> FALL : 하락 | String |
/// | change_price | 변화액의 절대값 | Double |
/// | change_rate | 변화율의 절대값 | Double |
/// | signed_change_price | 부호가 있는 변화액 | Double |
/// | signed_change_rate | 부호가 있는 변화율 | Double |
/// | trade_volume | 가장 최근 거래량 | Double |
/// | acc_trade_price | 누적 거래대금(UTC 0시 기준) | Double |
/// | acc_trade_price_24h | 24시간 누적 거래대금 | Double |
/// | acc_trade_volume | 누적 거래량(UTC 0시 기준) | Double |
/// | acc_trade_volume_24h | 24시간 누적 거래량 | Double |
/// | highest_52_week_price | 52주 신고가 | Double |
/// | highest_52_week_date | 52주 신고가 달성일 <br> 포맷: yyyy-MM-dd | String |
/// | lowest_52_week_price | 52주 신저가 | Double |
/// | lowest_52_week_date | 52주 신저가 달성일 <br> 포맷: yyyy-MM-dd | String |
/// | timestamp | 타임스탬프 | Long |
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