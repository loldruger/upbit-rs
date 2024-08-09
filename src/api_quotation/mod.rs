pub mod candle_day;
pub mod candle_minute;
pub mod candle_month;
pub mod candle_week;
pub mod market_state;
pub mod order_book;
pub mod ticker_snapshot;
pub mod trade_recent;

use std::fmt::Display;

pub use candle_day::CandleChartDay;
pub use candle_minute::CandleChartMinute;
pub use candle_month::CandleChartMonth;
pub use candle_week::CandleChartWeek;
pub use market_state::MarketState;
pub use order_book::OrderBookInfo;
use serde::Deserialize;
pub use ticker_snapshot::TickerSnapshot;
pub use trade_recent::TradeRecent;

use crate::constant::{URL_CANDLE_DAY, URL_CANDLE_MINUTE, URL_CANDLE_MONTH, URL_CANDLE_WEEK};
use crate::response::ResponseError;

#[cfg(feature = "sqlx-type")]
use sqlx::Type;

/// Kind of change of ticker snapshot
#[derive(Deserialize, Debug, Copy, Clone)]
#[cfg_attr(feature = "sqlx-type", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx-type", sqlx(type_name = "snapshot_change_type"), sqlx(rename_all = "snake_case"))]
pub enum SnapshotChangeType {
    /// 보합
    Even,
    /// 상승
    Rise,
    /// 하락
    Fall,
}

impl From<&str> for SnapshotChangeType {
    fn from(value: &str) -> Self {
        match value {
            "EVEN" => Self::Even,
            "FALL" => Self::Fall,
            "RISE" => Self::Rise,
            a @ _ => panic!("Unexpected value: {}", a),
        }
    }
}

/// Kind of minute unit of minute candle chart
#[derive(Clone, Copy)]
pub enum CandleMinute {
    /// Into() coerces it into u8 sized 1
    Min1,
    /// Into() coerces it into u8 sized 3
    Min3,
    /// Into() coerces it into u8 sized 5
    Min5,
    /// Into() coerces it into u8 sized 10
    Min10,
    /// Into() coerces it into u8 sized 15
    Min15,
    /// Into() coerces it into u8 sized 30
    Min30,
    /// Into() coerces it into u8 sized 60
    Min60,
    /// Into() coerces it into u8 sized 240
    Min240,
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

/// For the purpose of descripting the kind of candle chart time
pub enum UrlAssociates {
    UrlCandleMinute(CandleMinute),
    UrlCandleWeek,
    UrlCandleDay,
    UrlCandleMonth,
}

impl Display for UrlAssociates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UrlAssociates::UrlCandleMinute(minute) => {
                write!(f, "{URL_CANDLE_MINUTE}{}", Into::<u8>::into(*minute))
            }
            UrlAssociates::UrlCandleDay => {
                write!(f, "{}", URL_CANDLE_DAY)
            }
            UrlAssociates::UrlCandleWeek => {
                write!(f, "{}", URL_CANDLE_WEEK)
            }
            UrlAssociates::UrlCandleMonth => {
                write!(f, "{}", URL_CANDLE_MONTH)
            }
        }
    }
}

/// 호가 정보를 조회한다. (Inquiry bid price and offered price.)
///
/// # Example
/// ```rust
/// let order_book_info = api_quotation::get_order_book_info(&["KRW-ETH"]).await;
/// ```
/// - parameters
/// > `market_id` ex) KRW-ETH<br>
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
/// # Response Description
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
pub async fn get_order_book_info(markets_id: &[&str]) -> Result<OrderBookInfo, ResponseError> {
    OrderBookInfo::get_orderbook_info(markets_id).await
}

/// 요청 당시 종목의 스냅샷을 반환한다. (Return the snapshot of the ticker at the moment of query.)
///
/// # Example
/// ```rust
/// let ticker_snapshot = api_quotation::get_ticker_snapshot(&["KRW-ETH"]).await;
/// ```
/// - parameters
/// > `market_id` ex) KRW-ETH<br>
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
/// # Response Description
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
pub async fn get_ticker_snapshot(market: &[&str]) -> Result<TickerSnapshot, ResponseError> {
    TickerSnapshot::get_ticker_snapshot(market).await
}

/// 호가 정보를 조회한다. (Inquiry bid price and offered price.)
///
/// # Example
/// ```rust
/// let recent_trade_list = api_quotation::get_trade_recent_list("KRW-ETH").await;
/// ```
/// - parameters
/// > `market_id` ex) KRW-ETH<br>
/// > `hhmmss` format is "HHmmss" or "HH:mm:ss". if empty, latest data will be retrieved<br>
/// > `count` count of trade<br>
/// > `cursor` pagenation cursor. (sequential id)<br>
/// > `days_ago`You can retrieve previous data within 7 days based on the recent transaction date. If left empty, the most recent transaction date is returned. (Range: 1 ~ 7))<br>
/// # Response
/// ```json
/// [
///   {
///     "market": "KRW-BTC",
///     "trade_date_utc": "2018-04-18",
///     "trade_time_utc": "10:19:58",
///     "timestamp": 1524046798000,
///     "trade_price": 8616000,
///     "trade_volume": 0.03060688,
///     "prev_closing_price": 8450000,
///     "chane_price": 166000,
///     "ask_bid": "ASK"
///   }
/// ]
/// ```
/// # Response Description
/// | field             | description                   | type         |
/// |:------------------|:------------------------------|:-------------|
/// | market | 마켓 구분 코드 | String |
/// | trade_date_utc | 체결 일자(UTC 기준) <br> 포맷: yyyy-MM-dd | String
/// | trade_time_utc | 체결 시각(UTC 기준) <br> 포맷: HH:mm:ss | String
/// | timestamp | 체결 타임스탬프 | Long |
/// | trade_price | 체결 가격 | Double |
/// | trade_volume | 체결량 | Double |
/// | prev_closing_price | 전일 종가(UTC 0시 기준) | Double |
/// | change_price | 변화량 | Double |
/// | ask_bid | 매도/매수 | String |
/// | sequential_id | 체결 번호(Unique) | Long |
///
/// * sequential_id 필드는 체결의 유일성 판단을 위한 근거로 쓰일 수 있습니다. 하지만 체결의 순서를 보장하지는 못합니다.
pub async fn get_trade_recent_list(
    market: &str,
    hhmmss: Option<&str>,
    count: i32,
    cursor: &str,
    days_ago: Option<i32>,
) -> Result<TradeRecent, ResponseError> {
    TradeRecent::get_trade_recent_list(market, hhmmss, count, cursor, days_ago).await
}

/// 업비트에서 거래 가능한 마켓 목록 (List of markets available on Upbit)
///
/// # Example
/// ```
/// let market_state = api_quotation::get_market_state(true).await;
/// ```
/// - parameters
/// > `is_detailed` 유의종목 필드과 같은 상세 정보 노출 여부(선택 파라미터) <br>
/// # Response
/// ```json
/// [
///     {
///         "market": "KRW-BTC",
///         "korean_name": "비트코인",
///         "english_name": "Bitcoin"
///     },
///     ...
/// ]
/// ```
/// # Response Description
/// | field             | description                   | type         |
/// |:------------------|:------------------------------|:-------------|
/// | market | 업비트에서 제공중인 시장 정보 | String |
/// | korean_name | 거래 대상 디지털 자산 한글명 | String |
/// | english_name | 거래 대상 디지털 자산 영문명 | String |
/// | market_warning | 유의 종목 여부 <br> NONE: (해당 사항 없음), CAUTION(투자유의) | String |
pub async fn get_market_state(is_detailed: bool) -> Result<Vec<MarketState>, ResponseError> {
    MarketState::get_market_state(is_detailed).await
}

/// 분봉 캔들 데이터를 요청한다. (inquire minute-unit candle data.)
///
/// # Example
/// ```
/// let candle_of_minute = api_quotation::get_candle_minute("KRW-ETH", None, 10, CandleMinute::Min30).await;
/// ```
/// - parameters
/// > `market_id` ex) KRW-ETH<br>
/// > `to` the time moment of the last candle (exclusive). if empty, latest candle will be retrived. <br>
///  >> *  ISO8061 format (yyyy-MM-dd'T'HH:mm:ss'Z' or yyyy-MM-dd HH:mm:ss). <br>
///  >> *  though it is commonly UTC time criteria, you can request KST time using like 2023-01-01T00:00:00+09:00 format. <br>
///
/// > `count` the number of candle to request. maximum value: `200`<br>
/// > `candle_minute` unit of minute
///  >> *  `CandleMinute::Min1`<br>
///  >> *  `CandleMinute::Min3`<br>
///  >> *  `CandleMinute::Min5`<br>
///  >> *  `CandleMinute::Min10`<br>
///  >> *  `CandleMinute::Min15`<br>
///  >> *  `CandleMinute::Min30`<br>
///  >> *  `CandleMinute::Min60`<br>
///  >> *  `CandleMinute::Min240`
/// # Response
/// ```json
/// [
///   {
///     "market": "KRW-BTC",
///     "candle_date_time_utc": "2018-04-18T10:16:00",
///     "candle_date_time_kst": "2018-04-18T19:16:00",
///     "opening_price": 8615000,
///     "high_price": 8618000,
///     "low_price": 8611000,
///     "trade_price": 8616000,
///     "timestamp": 1524046594584,
///     "candle_acc_trade_price": 60018891.90054,
///     "candle_acc_trade_volume": 6.96780929,
///     "unit": 1
///   }
/// ]
/// ```
/// # Response Description
/// | field             | description                   | type         |
/// |:------------------|:------------------------------|:-------------|
/// | market | 마켓명 | String |
/// | candle_date_time_utc | 캔들 기준 시각(UTC 기준) <br> 포맷: yyyy-MM-dd'T'HH:mm:ss | String |
/// | candle_date_time_kst | 캔들 기준 시각(KST 기준) <br> 포맷: yyyy-MM-dd'T'HH:mm:ss | String |
/// | opening_price | 시가 | Double |
/// | high_price | 고가 | Double |
/// | low_price | 저가 | Double |
/// | trade_price | 종가 | Double |
/// | timestamp | 해당 캔들에서 마지막 틱이 저장된 시각 | Long |
/// | candle_acc_trade_price | 누적 거래 금액 | Double |
/// | candle_acc_trade_volume | 누적 거래량 | Double |
/// | unit | 분 단위(유닛) | Integer |
pub async fn get_candle_minute(
    market: &str,
    to: Option<String>,
    count: i32,
    candle_minute: CandleMinute,
) -> Result<Vec<CandleChartMinute>, ResponseError> {
    CandleChartMinute::request_candle(market, to, count, candle_minute).await
}

/// 일봉 캔들 데이터를 요청한다. (inquire day-unit candle data.)
///
/// # Example
/// ```
/// let candle_of_day = api_quotation::get_candle_day("KRW-ETH", 10, None, None).await;
/// ```
/// - parameters
/// > `market_id` ex) KRW-ETH<br>
/// > `count` the number of candle to request. maximum value: `200`<br>
/// > `last_candle_time` (optional) the time moment of the last candle (exclusive). if empty, latest candle will be retrived. <br>
/// >> * ISO8061 format (yyyy-MM-dd'T'HH:mm:ss'Z' or yyyy-MM-dd HH:mm:ss). <br>
/// >> * though it is commonly UTC time criteria, you can request KST time using like 2023-01-01T00:00:00+09:00 format. <br>
///
/// > `price_unit` (optional)
/// # Response
/// ```json
/// [
///   {
///     "market": "KRW-BTC",
///     "candle_date_time_utc": "2018-04-18T00:00:00",
///     "candle_date_time_kst": "2018-04-18T09:00:00",
///     "opening_price": 8450000,
///     "high_price": 8679000,
///     "low_price": 8445000,
///     "trade_price": 8626000,
///     "timestamp": 1524046650532,
///     "candle_acc_trade_price": 107184005903.68721,
///     "candle_acc_trade_volume": 12505.93101659,
///     "prev_closing_price": 8450000,
///     "change_price": 176000,
///     "change_rate": 0.0208284024
///   }
/// ]
/// ```
/// # Response Description
/// | field             | description                   | type         |
/// |:------------------|:------------------------------|:-------------|
/// | market | 마켓명 | String |
/// | candle_date_time_utc | 캔들 기준 시각(UTC 기준) <br> 포맷: yyyy-MM-dd'T'HH:mm:ss | String |
/// | candle_date_time_kst | 캔들 기준 시각(KST 기준) <br> 포맷: yyyy-MM-dd'T'HH:mm:ss | String |
/// | opening_price | 시가 | Double |
/// | high_price | 고가 | Double |
/// | low_price | 저가 | Double |
/// | trade_price | 종가 | Double |
/// | timestamp | 마지막 틱이 저장된 시각 | Long |
/// | candle_acc_trade_price | 누적 거래 금액 | Double |
/// | candle_acc_trade_volume | 누적 거래량 | Double |
/// | prev_closing_price | 전일 종가(UTC 0시 기준) | Double |
/// | change_price | 전일 종가 대비 변화 금액 | Double |
/// | change_rate | 전일 종가 대비 변화량 | Double |
/// | converted_trade_price | 종가 환산 화폐 단위로 환산된 가격(요청에 convertingPriceUnit 파라미터 없을 시 해당 필드 포함되지 않음.) | Double |
pub async fn get_candle_day(
    market: &str,
    count: i32,
    last_candle_time: Option<String>,
    price_unit: Option<String>,
) -> Result<Vec<CandleChartDay>, ResponseError> {
    CandleChartDay::request_candle(market, count, last_candle_time, price_unit).await
}

/// 주봉 캔들 데이터를 요청한다. (inquire week-unit candle data.)
///
/// # Example
/// ```
/// let candle_of_week = api_quotation::get_candle_week("KRW-ETH", 10, None).await;
/// ```
/// - parameters
/// > `market_id` ex) KRW-ETH<br>
/// > `count` the number of candle to request. maximum value: `200`<br>
/// > `last_candle_time` (optional) the time moment of the last candle (exclusive). if empty, latest candle will be retrived. <br>
/// >> *  `ISO8061` format (`yyyy-MM-dd'T'HH:mm:ss'Z'` or `yyyy-MM-dd HH:mm:ss`). <br>
/// >> *  though it is commonly UTC time criteria, you can request KST time using like `2023-01-01T00:00:00+09:00` format. <br>
///
/// # Response
/// ```json
/// [
///   {
///     "market": "KRW-BTC",
///     "candle_date_time_utc": "2018-04-16T00:00:00",
///     "candle_date_time_kst": "2018-04-16T09:00:00",
///     "opening_price": 8665000,
///     "high_price": 8840000,
///     "low_price": 8360000,
///     "trade_price": 8611000,
///     "timestamp": 1524046708995,
///     "candle_acc_trade_price": 466989414916.1301,
///     "candle_acc_trade_volume": 54410.56660813,
///     "first_day_of_period": "2018-04-16"
///   }
/// ]
/// ```
/// # Response Description
/// | field             | description                   | type         |
/// |:------------------|:------------------------------|:-------------|
/// | market | 마켓명 | String |
/// | candle_date_time_utc | 캔들 기준 시각(UTC 기준) <br> 포맷: yyyy-MM-dd'T'HH:mm:ss | String |
/// | candle_date_time_kst | 캔들 기준 시각(KST 기준) <br> 포맷: yyyy-MM-dd'T'HH:mm:ss | String |
/// | opening_price | 시가 | Double |
/// | high_price | 고가 | Double |
/// | low_price | 저가 | Double |
/// | trade_price | 종가 | Double |
/// | timestamp | 마지막 틱이 저장된 시각 | Long |
/// | candle_acc_trade_price | 누적 거래 금액 | Double |
/// | candle_acc_trade_volume | 누적 거래량 | Double |
/// | first_day_of_period | 캔들 기간의 가장 첫 날 | String |
pub async fn get_candle_week(
    market: &str,
    count: i32,
    last_candle_time: Option<String>,
) -> Result<Vec<CandleChartWeek>, ResponseError> {
    CandleChartWeek::request_candle(market, count, last_candle_time).await
}

/// 월봉 캔들 데이터를 요청한다. (inquire month-unit candle data.)
///
/// # Example
/// ```
/// let candle_of_month = api_quotation::get_candle_month("KRW-ETH", 10, None).await;
/// ```
/// - parameters
/// > `market_id` ex) KRW-ETH<br>
/// > `count` the number of candle to request. maximum value: `200`<br>
/// > `last_candle_time` (optional) the time moment of the last candle (exclusive). if empty, latest candle will be retrived. <br>
///  >> *  ISO8061 format (yyyy-MM-dd'T'HH:mm:ss'Z' or yyyy-MM-dd HH:mm:ss). <br>
///  >> *  though it is commonly UTC time criteria, you can request KST time using like 2023-01-01T00:00:00+09:00 format. <br>
///
/// # Response
/// ```json
/// [
///   {
///     "market": "KRW-BTC",
///     "candle_date_time_utc": "2018-04-01T00:00:00",
///     "candle_date_time_kst": "2018-04-01T09:00:00",
///     "opening_price": 7688000,
///     "high_price": 8840000,
///     "low_price": 7087000,
///     "trade_price": 8614000,
///     "timestamp": 1524046761201,
///     "candle_acc_trade_price": 2665448149094.0195,
///     "candle_acc_trade_volume": 336501.67751807,
///     "first_day_of_period": "2018-04-01"
///   }
/// ]
/// ```
/// # Response Description
/// | field             | description                   | type         |
/// |:------------------|:------------------------------|:-------------|
/// | market | 마켓명 | String |
/// | candle_date_time_utc | 캔들 기준 시각(UTC 기준) <br> 포맷: yyyy-MM-dd'T'HH:mm:ss | String |
/// | candle_date_time_kst | 캔들 기준 시각(KST 기준) <br> 포맷: yyyy-MM-dd'T'HH:mm:ss | String |
/// | opening_price | 시가 | Double |
/// | high_price | 고가 | Double |
/// | low_price | 저가 | Double |
/// | trade_price | 종가 | Double |
/// | timestamp | 마지막 틱이 저장된 시각 | Long |
/// | candle_acc_trade_price | 누적 거래 금액 | Double |
/// | candle_acc_trade_volume | 누적 거래량 | Double |
/// | first_day_of_period | 캔들 기간의 가장 첫 날 | String |
pub async fn get_candle_month(
    market: &str,
    count: i32,
    last_candle_time: Option<String>,
) -> Result<Vec<CandleChartMonth>, ResponseError> {
    CandleChartMonth::request_candle(market, count, last_candle_time).await
}
