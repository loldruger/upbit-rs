pub mod accounts;
pub mod order;
pub mod order_cancel;
pub mod order_chance;
pub mod order_status;
pub mod order_status_list;

use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::constant::OrderBy;

use super::response::{AccountsInfo, OrderInfo, OrderChance, OrderStatus, ResponseError};

/// Side of order
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
pub enum OrderSide {
    /// 매수
    Bid, 
    /// 매도
    Ask,
}

impl Display for OrderSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderSide::Bid => write!(f, "bid"),
            OrderSide::Ask => write!(f, "ask"),
        }
    }
}

// impl ToString for OrderSide {
//     fn to_string(&self) -> String {
//         match self {
//             OrderSide::Bid => "bid".to_owned(),
//             OrderSide::Ask => "ask".to_owned(),
//         }
//     }
// }

impl From<&str> for OrderSide {
    fn from(value: &str) -> Self {
        match value {
            "bid" => OrderSide::Bid,
            "ask" => OrderSide::Ask,
            _ => panic!("value must be either \"bid\" or \"ask!\"")
        }
    }
}

/// Type of order
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
pub enum OrderType {
    /// 지정가 주문
    Limit,
    /// 시장가 주문(매수)
    Price,
    /// 시장가 주문(매도)
    Market,
    /// 최유리 주문
    Best
}

impl Display for OrderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderType::Limit => write!(f, "limit"),
            OrderType::Price => write!(f, "price"),
            OrderType::Market => write!(f, "market"),
            OrderType::Best => write!(f, "best"),
        }
    }
}

// impl ToString for OrderType {
//     fn to_string(&self) -> String {
//         match self {
//             OrderType::Limit => "limit".to_owned(),
//             OrderType::Price => "price".to_owned(),
//             OrderType::Market => "market".to_owned(),
//             OrderType::Best => "best".to_owned(),
//         }
//     }
// }

impl From<&str> for OrderType {
    fn from(value: &str) -> Self {
        match value {
            "limit" => OrderType::Limit,
            "price" => OrderType::Price,
            "market" => OrderType::Market,
            "best" => OrderType::Best,
            _ => panic!("value must be one of \"limit\", \"price!\", \"market\" or \"best\".")
        }
    }
}

/// New Order type 
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum OrderCondition {
    /// Immediate or Cancel
    IOK,
    /// Fill or Kill
    FOK,
}

impl Display for OrderCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderCondition::IOK => write!(f, "iok"),
            OrderCondition::FOK => write!(f, "fok")
        }
    }
}

// impl ToString for OrderCondition {
//     fn to_string(&self) -> String {
//         match self {
//             OrderCondition::IOK => "iok".to_owned(),
//             OrderCondition::FOK => "fok".to_owned()
//         }
//     }
// }

impl From<&str> for OrderCondition {
    fn from(value: &str) -> Self {
        match value {
            "iok" => OrderCondition::IOK,
            "fok" => OrderCondition::FOK,
            _ => panic!("value must be one of \"iok\" or \"fok\".")
        }
    }
}

/// List of order state
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum OrderState {
    /// 체결 대기 (default)
    Wait,
    /// 예약주문 대기
    Watch,
    /// 전체 체결 완료
    Done,
    /// 주문 취소
    Cancel
}

impl Display for OrderState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderState::Wait => write!(f, "wait"),
            OrderState::Watch => write!(f, "watch"),
            OrderState::Done => write!(f, "done"),
            OrderState::Cancel => write!(f, "cancel"),
        }
    }
}

// impl ToString for OrderState {
//     fn to_string(&self) -> String {
//         match self {
//             OrderState::Wait => "wait".to_owned(),
//             OrderState::Watch => "watch".to_owned(),
//             OrderState::Done => "done".to_owned(),
//             OrderState::Cancel => "cancel".to_owned(),
//         }
//     }
// }

impl From<&str> for OrderState {
    fn from(value: &str) -> Self {
        match value {
            "wait" => OrderState::Wait,
            "watch" => OrderState::Watch,
            "done" => OrderState::Done,
            "cancel" => OrderState::Cancel,
            _ => panic!("value must be one of \"wait\", \"watch!\", \"done!\" or \"cancel\"")
        }
    }
}

/// 주문 요청을 한다. (Make an order(buy or sell) with desired price )
/// 
/// # Example
/// ```
/// let order_info = api_exchange::order_by_price("KRW-ETH", OrderSide::Bid, 5000.0, 1_435_085.0, OrderType::Limit, None).await;
/// ```
/// - parameters
/// > `market` ex) "KRW-ETH" <br>
/// > `side` 
/// >> *  `OrderSide::BID` 매수<br>
/// >> *  `OrderSide::ASK` 매도<br>
/// 
/// > `price` price that how much you want to buy<br>
/// > `price_desired` price that you want to bid at<br>
/// > `ord_type` 
/// >> *  `OrderType::LIMIT` 지정가 주문<br>
/// >> *  `OrderType::PRICE` 시장가 주문(매수)<br>
/// >> *  `OrderType::MARKET` 시장가 주문(매도)<br>
/// 
/// > `identifier` (optional) specific identifier you have tagged<br>
/// # Response
/// ```json
/// {
///    "uuid": "cdd92199-2897-4e14-9448-f923320408ad",
///    "side": "bid",
///    "ord_type": "limit",
///    "price": "100.0",
///    "state": "wait",
///    "market": "KRW-BTC",
///    "created_at": "2018-04-10T15:42:23+09:00",
///    "volume": "0.01",
///    "remaining_volume": "0.01",
///    "reserved_fee": "0.0015",
///    "remaining_fee": "0.0015",
///    "paid_fee": "0.0",
///    "locked": "1.0015",
///    "executed_volume": "0.0",
///    "trades_count": 0
///  }
/// ```
/// | field             | description                   | type         |
/// |:------------------|:------------------------------|:-------------|
/// | uuid              | 주문의 고유 아이디             | String |
/// | side              | 주문 종류                     | String |
/// | ord_type          | 주문 방식                     | String |
/// | price             | 주문 당시 화폐 가격           | NumberString |
/// | state             | 주문 상태                     | String |
/// | market            | 마켓의 유일키                 | String |
/// | created_at        | 주문 생성 시간                | String |
/// | volume            | 사용자가 입력한 주문 양       | NumberString |
/// | remaining_volume  | 체결 후 남은 주문 양          | NumberString |
/// | reserved_fee      | 수수료로 예약된 비용          | NumberString |
/// | remaining_fee     | 남은 수수료                   | NumberString |
/// | paid_fee          | 사용된 수수료                | NumberString |
/// | locked            | 거래에 사용중인 비용          | NumberString |
/// | executed_volume   | 체결된 양                    | NumberString |
/// | trades_count      | 해당 주문에 걸린 체결 수      | Integer |

pub async fn order_by_price(
    market_id: &str,
    side: OrderSide,
    price: f64,
    price_desired: f64,
    ord_type: OrderType,
    identifier: Option<&str>,
) -> Result<OrderInfo, ResponseError> {
    OrderInfo::order_by_price(
        market_id,
        side,
        (price + 1.) / price_checker(price_desired),
        price_checker(price_desired),
        ord_type,
        identifier
    )
    .await
}


/// 즉시 시장가 판매를 한다. (Sell specific amount of volume immediately at market price.)
/// 
/// # Example
/// ```
/// let order_info = api_exchange::sell_by_market_price("KRW-ETH", 1.0, None).await;
/// let order_info = api_exchange::sell_by_market_price("KRW-ETH", 1.0, Some("cdd92199-2897-4e14-9448-f923320408ad")).await;
/// ```
/// - parameters
/// > `market` ex) "KRW-ETH" <br>
/// > `volume` volume you want to sell<br>
/// > `identifier` (optional) specific identifier you have tagged<br>
/// # Response
/// ```json
/// {
///    "uuid": "cdd92199-2897-4e14-9448-f923320408ad",
///    "side": "bid",
///    "ord_type": "limit",
///    "price": "100.0",
///    "state": "wait",
///    "market": "KRW-BTC",
///    "created_at": "2018-04-10T15:42:23+09:00",
///    "volume": "0.01",
///    "remaining_volume": "0.01",
///    "reserved_fee": "0.0015",
///    "remaining_fee": "0.0015",
///    "paid_fee": "0.0",
///    "locked": "1.0015",
///    "executed_volume": "0.0",
///    "trades_count": 0
///  }
/// ```
/// | field             | description                   | type         |
/// |:------------------|:------------------------------|:-------------|
/// | uuid              | 주문의 고유 아이디             | String |
/// | side              | 주문 종류                     | String |
/// | ord_type          | 주문 방식                     | String |
/// | price             | 주문 당시 화폐 가격           | NumberString |
/// | state             | 주문 상태                     | String |
/// | market            | 마켓의 유일키                 | String |
/// | created_at        | 주문 생성 시간                | String |
/// | volume            | 사용자가 입력한 주문 양       | NumberString |
/// | remaining_volume  | 체결 후 남은 주문 양          | NumberString |
/// | reserved_fee      | 수수료로 예약된 비용          | NumberString |
/// | remaining_fee     | 남은 수수료                   | NumberString |
/// | paid_fee          | 사용된 수수료                | NumberString |
/// | locked            | 거래에 사용중인 비용          | NumberString |
/// | executed_volume   | 체결된 양                    | NumberString |
/// | trades_count      | 해당 주문에 걸린 체결 수      | Integer |

pub async fn sell_at_market_price(market_id: &str, volume: f64, identifier: Option<&str>) -> Result<OrderInfo, ResponseError> {
    OrderInfo::order_ask_at_market_price(
        market_id,
        OrderSide::Ask,
        volume,
        OrderType::Market,
        identifier
    )
    .await
}

/// 주문을 취소한다. (Cancel an order.)
/// 
/// # Example
/// ```
/// let order_info = api_exchange::cancel_order("cdd92199-2897-4e14-9448-f923320408ad", None).await;
/// ```
/// - parameters
/// > `uuid` (optional) uuid of order to cancel <br>
/// > `identifier` (optional) specific identifier you have tagged<br>
/// 
/// * One of the two parameter must be input. Error when both parameter are entered or neither parameter are entered.
/// # Response
/// ```json
/// {
///    "uuid": "cdd92199-2897-4e14-9448-f923320408ad",
///    "side": "bid",
///    "ord_type": "limit",
///    "price": "100.0",
///    "state": "wait",
///    "market": "KRW-BTC",
///    "created_at": "2018-04-10T15:42:23+09:00",
///    "volume": "0.01",
///    "remaining_volume": "0.01",
///    "reserved_fee": "0.0015",
///    "remaining_fee": "0.0015",
///    "paid_fee": "0.0",
///    "locked": "1.0015",
///    "executed_volume": "0.0",
///    "trades_count": 0
///  }
/// ```
/// | field             | description                   | type         |
/// |:------------------|:------------------------------|:-------------|
/// | uuid              | 주문의 고유 아이디             | String |
/// | side              | 주문 종류                     | String |
/// | ord_type          | 주문 방식                     | String |
/// | price             | 주문 당시 화폐 가격           | NumberString |
/// | state             | 주문 상태                     | String |
/// | market            | 마켓의 유일키                 | String |
/// | created_at        | 주문 생성 시간                | String |
/// | volume            | 사용자가 입력한 주문 양       | NumberString |
/// | remaining_volume  | 체결 후 남은 주문 양          | NumberString |
/// | reserved_fee      | 수수료로 예약된 비용          | NumberString |
/// | remaining_fee     | 남은 수수료                   | NumberString |
/// | paid_fee          | 사용된 수수료                | NumberString |
/// | locked            | 거래에 사용중인 비용          | NumberString |
/// | executed_volume   | 체결된 양                    | NumberString |
/// | trades_count      | 해당 주문에 걸린 체결 수      | Integer |
pub async fn cancel_order_by_uuid(uuid: &str) -> Result<OrderInfo, ResponseError> {
    OrderInfo::cancel_order_by_uuid(uuid).await
}

pub async fn cancel_order_by_identifier(identifier: &str) -> Result<OrderInfo, ResponseError> {
    OrderInfo::cancel_order_by_identifier(identifier).await
}

/// 내가 보유한 자산 리스트를 보여줍니다. (inquire your account info)
/// 
/// # Example
/// ```
/// let order_info = api_exchange::get_account_info().await;
/// ```
/// # Response
/// ```json
/// [
///   {
///     "currency":"KRW",
///     "balance":"1000000.0",
///     "locked":"0.0",
///     "avg_buy_price":"0",
///     "avg_buy_price_modified":false,
///     "unit_currency": "KRW",
///   },
///   {
///     "currency":"BTC",
///     "balance":"2.0",
///     "locked":"0.0",
///     "avg_buy_price":"101000",
///     "avg_buy_price_modified":false,
///     "unit_currency": "KRW",
///   }
/// ]
/// ```
/// | field                  | description                   | type         |
/// |:-----------------------|:------------------------------|:-------------|
/// | currency               | 화폐를 의미하는 영문 대문자 코드 | String       |
/// | balance                | 주문가능 금액/수량              | NumberString |
/// | locked                 | 주문 중 묶여있는 금액/수량      | NumberString |
/// | avg_buy_price          | 매수평균가                     | NumberString |
/// | avg_buy_price_modified | 매수평균가 수정 여부            | Boolean      |
/// | unit_currency          | 평단가 기준 화폐                | String       |
pub async fn get_account_info() -> Result<Vec<AccountsInfo>, ResponseError> {
    AccountsInfo::get_account_info().await
}

/// 마켓별 주문 가능 정보를 확인한다. (check specific market status.)
/// 
/// # Example
/// ```
/// let order_chance = api_exchange::get_order_chance("KRW-ETH").await;
/// ```
/// - parameters
/// > `market` ex) KRW-ETH<br>
/// 
/// # Response
/// ```json
/// {
///   "bid_fee": "0.0015",
///   "ask_fee": "0.0015",
///   "market": {
///     "id": "KRW-BTC",
///     "name": "BTC/KRW",
///     "order_types": [
///       "limit"
///     ],
///     "order_sides": [
///       "ask",
///       "bid"
///     ],
///     "bid": {
///       "currency": "KRW",
///       "price_unit": null,
///       "min_total": 1000
///     },
///     "ask": {
///       "currency": "BTC",
///       "price_unit": null,
///       "min_total": 1000
///     },
///     "max_total": "100000000.0",
///     "state": "active",
///   },
///   "bid_account": {
///     "currency": "KRW",
///     "balance": "0.0",
///     "locked": "0.0",
///     "avg_buy_price": "0",
///     "avg_buy_price_modified": false,
///     "unit_currency": "KRW",
///   },
///   "ask_account": {
///     "currency": "BTC",
///     "balance": "10.0",
///     "locked": "0.0",
///     "avg_buy_price": "8042000",
///     "avg_buy_price_modified": false,
///     "unit_currency": "KRW",
///   }
/// }
/// ```
/// | field                  | description                   | type         |
/// |:-----------------------|:------------------------------|:-------------|
/// | bid_fee |매수 수수료 비율 | NumberString |
/// | ask_fee| 매도 수수료 비율 | NumberString |
/// | market| 마켓에 대한 정보 | Object |
/// | market.id| 마켓의 유일 키 | String |
/// | market.name| 마켓 이름 | String |
/// | market.order_types| 지원 주문 방식 (만료) | Array[[String]] |
/// | ask_types| 매도 주문 지원 방식 | Array[[String]] |
/// | bid_types| 매수 주문 지원 방식 | Array[[String]] |
/// | market.order_sides| 지원 주문 종류 | Array[[String]] |
/// | market.bid| 매수 시 제약사항 | Object |
/// | market.bid.currency| 화폐를 의미하는 영문 대문자 코드 | String |
/// | market.bid.price_unit| 주문금액 단위 | String |
/// | market.bid.min_total| 최소 매도/매수 금액 | Number |
/// | market.ask| 매도 시 제약사항 | Object |
/// | market.ask.currency| 화폐를 의미하는 영문 대문자 코드 | String |
/// | market.ask.price_unit| 주문금액 단위 | String |
/// | market.ask.min_total| 최소 매도/매수 금액 | Number |
/// | market.max_total| 최대 매도/매수 금액 | NumberString |
/// | market.state| 마켓 운영 상태 | String |
/// | bid_account| 매수 시 사용하는 화폐의 계좌 상태 | Object |
/// | bid_account.currency| 화폐를 의미하는 영문 대문자 코드 | String |
/// | bid_account.balance| 주문가능 금액/수량 | NumberString |
/// | bid_account.locked| 주문 중 묶여있는 금액/수량 | NumberString |
/// | bid_account.avg_buy_price| 매수평균가 | NumberString |
/// | bid_account.avg_buy_price_modified| 매수평균가 수정 여부 | Boolean |
/// | bid_account.unit_currency| 평단가 기준 화폐 | String |
/// | ask_account| 매도 시 사용하는 화폐의 계좌 상태 | Object |
/// | ask_account.currency| 화폐를 의미하는 영문 대문자 코드 | String |
/// | ask_account.balance| 주문가능 금액/수량 | NumberString |
/// | ask_account.locked| 주문 중 묶여있는 금액/수량 | NumberString |
/// | ask_account.avg_buy_price| 매수평균가 | NumberString |
/// | ask_account.avg_buy_price_modified| 매수평균가 수정 여부 | Boolean |
/// | ask_account.unit_currency| 평단가 기준 화폐 | String |
pub async fn get_order_chance(market_id: &str) -> Result<OrderChance, ResponseError> {
    OrderChance::get_order_chance(market_id).await
}

/// 주문 UUID 를 통해 개별 주문건을 조회한다. (inquire each order status via order UUID.)
/// 
/// # Example
/// ```
/// let order_status = api_exchange::get_order_status("9ca023a5-851b-4fec-9f0a-48cd83c2eaae", None).await;
/// ```
/// - parameters
/// > `uuid` (optional) uuid of order to cancel <br>
/// > `identifier` (optional) specific identifier you have tagged<br>
/// 
/// * One of the two parameter must be input. Error when both parameter are entered or neither parameter are entered.
/// # Response
/// ```json
/// {
///   "uuid": "9ca023a5-851b-4fec-9f0a-48cd83c2eaae",
///   "side": "ask",
///   "ord_type": "limit",
///   "price": "4280000.0",
///   "state": "done",
///   "market": "KRW-BTC",
///   "created_at": "2019-01-04T13:48:09+09:00",
///   "volume": "1.0",
///   "remaining_volume": "0.0",
///   "reserved_fee": "0.0",
///   "remaining_fee": "0.0",
///   "paid_fee": "2140.0",
///   "locked": "0.0",
///   "executed_volume": "1.0",
///   "trades_count": 1,
///   "trades": [
///     {
///       "market": "KRW-BTC",
///       "uuid": "9e8f8eba-7050-4837-8969-cfc272cbe083",
///       "price": "4280000.0",
///       "volume": "1.0",
///       "funds": "4280000.0",
///       "side": "ask"
///     }
///   ]
/// }
/// ```
/// | field                  | description                   | type         |
/// |:-----------------------|:------------------------------|:-------------|
/// | uuid | 주문의 고유 아이디 | String |
/// | side | 주문 종류 | String |
/// | ord_type | 주문 방식 | String |
/// | price | 주문 당시 화폐 가격 | NumberString |
/// | state | 주문 상태 | String |
/// | market | 마켓의 유일키 | String |
/// | created_at | 주문 생성 시간 | DateString |
/// | volume | 사용자가 입력한 주문 양 | NumberString |
/// | remaining_volume |체결 후 남은 주문 양 | NumberString |
/// | reserved_fee | 수수료로 예약된 비용 | NumberString |
/// | remaining_fee | 남은 수수료 | NumberString |
/// | paid_fee | 사용된 수수료 | NumberString |
/// | locked | 거래에 사용중인 비용 | NumberString |
/// | executed_volume | 체결된 양 | NumberString |
/// | trades_count |해당 주문에 걸린 체결 수 | Integer |
/// | trades | 체결 |Array[[Object]] |
/// | trades.market |마켓의 유일 키 | String |
/// | trades.uuid | 체결의 고유 아이디 | String |
/// | trades.price | 체결 가격 | NumberString |
/// | trades.volume | 체결 양 | NumberString |
/// | trades.funds | 체결된 총 가격 | NumberString |
/// | trades.side | 체결 종류 | String |
/// | trades.created_at | 체결 시각 | DateString |
pub async fn get_order_status_by_uuid(uuid: &str) -> Result<OrderStatus, ResponseError> {
    OrderStatus::get_order_status_by_uuid(uuid).await
}

pub async fn get_order_status_by_identifier(identifier: &str) -> Result<OrderStatus, ResponseError> {
    OrderStatus::get_order_status_by_identifier(identifier).await
}

/// 주문 리스트를 조회한다. (inquire every order status.)
/// 
/// # Example
/// ```
/// let order_status = api_exchange::list_order_status().await;
/// ```
/// ```json
/// [
///   {
///     "uuid": "9ca023a5-851b-4fec-9f0a-48cd83c2eaae",
///     "side": "ask",
///     "ord_type": "limit",
///     "price": "4280000.0",
///     "state": "done",
///     "market": "KRW-BTC",
///     "created_at": "2019-01-04T13:48:09+09:00",
///     "volume": "1.0",
///     "remaining_volume": "0.0",
///     "reserved_fee": "0.0",
///     "remaining_fee": "0.0",
///     "paid_fee": "2140.0",
///     "locked": "0.0",
///     "executed_volume": "1.0",
///     "trades_count": 1,
///   }
///   # ....
/// ]
/// ```
/// # Response
/// | field                  | description                   | type         |
/// |:-----------------------|:------------------------------|:-------------|
/// | uuid | 주문의 고유 아이디 | String |
/// | side | 주문 종류 | String |
/// | ord_type | 주문 방식 | String |
/// | price | 주문 당시 화폐 가격 | NumberString |
/// | state | 주문 상태 | String |
/// | market | 마켓의 유일키 | String |
/// | created_at | 주문 생성 시간 | DateString |
/// | volume | 사용자가 입력한 주문 양 | NumberString |
/// | remaining_volume | 체결 후 남은 주문 양 | NumberString |
/// | reserved_fee | 수수료로 예약된 비용 | NumberString |
/// | remaining_fee | 남은 수수료 | NumberString |
/// | paid_fee | 사용된 수수료 | NumberString |
/// | locked | 거래에 사용중인 비용 | NumberString |
/// | executed_volume | 체결된 양 | NumberString |
/// | trades_count | 해당 주문에 걸린 체결 수 | Integer |
#[deprecated(since = "1.6.0", note = "use get_order_status() instead")]
pub async fn list_order_status() -> Result<Vec<OrderInfo>, ResponseError> {
    OrderInfo::get_order_state_list().await
}

pub async fn get_order_states_by_uuids(market_id: &str, uuids: Vec<&str>, order_by: OrderBy) -> Result<Vec<OrderInfo>, ResponseError> {
    OrderInfo::get_order_states_by_uuids(market_id, uuids, order_by).await
}

pub async fn get_order_states_by_identifiers(market_id: &str, identifiers: Vec<&str>, order_by: OrderBy) -> Result<Vec<OrderInfo>, ResponseError> {
    OrderInfo::get_order_states_by_identifiers(market_id, identifiers, order_by).await
}

pub async fn get_order_states_opened(market_id: &str, state: OrderState, states: Vec<OrderState>, page: u8, limit: u8, order_by: OrderBy) -> Result<Vec<OrderInfo>, ResponseError> {
    OrderInfo::get_order_states_opened(market_id, state, states, page, limit, order_by).await
}

pub async fn get_order_states_closed(market_id: &str, state: OrderState, start_time: &str, end_time: &str, limit: u16, order_by: OrderBy) -> Result<Vec<OrderInfo>, ResponseError> {
    OrderInfo::get_order_states_closed(market_id, state, start_time, end_time, limit, order_by).await
}

pub fn price_checker(price: f64) -> f64 {
    let truncation = if price >= 2_000_000.0 { 1000.0 }
    else if price >= 1_000_000.0 { 500.0 }
    else if price >= 500_000.0 { 100.0 }
    else if price >= 100_000.0 { 50.0 }
    else if price >= 10_000.0 { 10.0 }
    else if price >= 1000.0 { 5.0 }
    else if price >= 100.0 { 1.0 }
    else if price >= 10.0 { 0.1 }
    else if price >= 0.0 { 0.01 }
    else { 0.001 };

    f64::trunc(price / truncation) * truncation
}
