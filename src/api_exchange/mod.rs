pub mod accounts;
pub mod order;
pub mod order_cancel;
pub mod order_chance;
pub mod order_status;
pub mod order_status_list;

use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::constant::OrderBy;

use super::response::{AccountsInfo, OrderChance, OrderInfo, OrderStatus, ResponseError};

#[cfg(feature = "sqlx-type")]
use sqlx::Type;

/// Side of order
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq)]

#[cfg_attr(
    feature = "sqlx-type",
    derive(sqlx::Type),
    sqlx(type_name = "order_side"),
    sqlx(rename_all = "snake_case")
)]
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

impl From<&str> for OrderSide {
    fn from(value: &str) -> Self {
        match value {
            "bid" => OrderSide::Bid,
            "ask" => OrderSide::Ask,
            a @ _ => panic!("Unexpected value: {}", a),
        }
    }
}

/// Type of order
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "sqlx-type", derive(sqlx::Type))]
#[cfg_attr(
    feature = "sqlx-type",
    sqlx(type_name = "order_type"),
    sqlx(rename_all = "snake_case")
)]
pub enum OrderType {
    /// 지정가 주문
    Limit,
    /// 시장가 주문(매수)
    Price,
    /// 시장가 주문(매도)
    Market,
    /// 최유리 주문
    Best,
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

impl From<&str> for OrderType {
    fn from(value: &str) -> Self {
        match value {
            "limit" => OrderType::Limit,
            "price" => OrderType::Price,
            "market" => OrderType::Market,
            "best" => OrderType::Best,
            a @ _ => panic!("Unexpected value: {}", a),
        }
    }
}

/// New Order type
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(feature = "sqlx-type", derive(sqlx::Type))]
#[cfg_attr(
    feature = "sqlx-type",
    sqlx(type_name = "order_condition"),
    sqlx(rename_all = "snake_case")
)]
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
            OrderCondition::FOK => write!(f, "fok"),
        }
    }
}

impl From<&str> for OrderCondition {
    fn from(value: &str) -> Self {
        match value {
            "iok" => OrderCondition::IOK,
            "fok" => OrderCondition::FOK,
            a @ _ => panic!("Unexpected value: {}", a),
        }
    }
}

/// List of order state
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(feature = "sqlx-type", derive(sqlx::Type))]
#[cfg_attr(
    feature = "sqlx-type",
    sqlx(type_name = "order_state"),
    sqlx(rename_all = "snake_case")
)]
pub enum OrderState {
    /// 체결 대기
    Wait,
    /// 예약주문 대기
    Watch,
    /// 전체 체결 완료
    Done,
    /// 주문 취소
    Cancel,
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

impl From<&str> for OrderState {
    fn from(value: &str) -> Self {
        match value {
            "wait" => OrderState::Wait,
            "watch" => OrderState::Watch,
            "done" => OrderState::Done,
            "cancel" => OrderState::Cancel,
            a @ _ => panic!("Unexpected value: {}", a),
        }
    }
}

/// 주문 요청을 한다. (Make an order(buy or sell) with desired price )
///
/// # Example
/// ```
/// let order_bid = api_exchange::order_by_price("KRW-ETH", OrderSide::Bid, 5000.0, 1_435_085.0, OrderType::Limit, None).await;
/// let order_ask = api_exchange::order_by_price("KRW-ETH", OrderSide::Ask, 5000.0, 10_435_085.0, OrderType::Limit, None).await;
/// ```
/// - parameters
/// > `market_id` ex) "KRW-ETH" <br>
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
/// # Response Description
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
        (price + 1.0) / price_checker(price_desired),
        price_checker(price_desired),
        ord_type,
        identifier,
    )
    .await
}

/// 주문을 취소한다. (Cancel an order.)
///
/// # Example
/// ```
/// let order_info = api_exchange::cancel_order_by_uuid("cdd92199-2897-4e14-9448-f923320408ad").await;
/// ```
/// - parameters
/// > `uuid` uuid of order to cancel <br>
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
/// # Response Description
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

/// 주문을 취소한다. (Cancel an order.)
///
/// # Example
/// ```
/// let order_info = api_exchange::cancel_order_by_identfiier("test_identfier").await;
/// ```
/// - parameters
/// > `identifier` specific identifier you have tagged<br>
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
/// # Response Description
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
/// # Response Description
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
/// > `market_id` ex) KRW-ETH<br>
///
/// # Response
/// ```json
/// {
///   "bid_fee": "0.0005",
///   "ask_fee": "0.0005",
///   "maker_bid_fee": "0.0005",
///   "maker_ask_fee": "0.0005",
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
///     "bid_types": [
///       "best_fok",
///       "best_ioc",
///       "limit",
///       "limit_fok",
///       "limit_ioc",
///       "price"
///     ],
///     "ask_types": [
///       "best_fok",
///       "best_ioc",
///       "limit",
///       "limit_fok",
///       "limit_ioc",
///       "market"
///     ],
///     "bid": {
///       "currency": "KRW",
///       "min_total": "5000"
///     },
///     "ask": {
///       "currency": "BTC",
///       "min_total": "5000"
///     },
///     "max_total": "1000000000",
///     "state": "active"
///   },
///   "bid_account": {
///     "currency": "KRW",
///     "balance": "0.61934932",
///     "locked": "0",
///     "avg_buy_price": "0",
///     "avg_buy_price_modified": true,
///     "unit_currency": "KRW"
///   },
///   "ask_account": {
///     "currency": "BTC",
///     "balance": "0.00001194",
///     "locked": "0",
///     "avg_buy_price": "88029000",
///     "avg_buy_price_modified": false,
///     "unit_currency": "KRW"
///   }
/// }
/// ```
/// # Response Description
/// | field                  | description                   | type         |
/// |:-----------------------|:------------------------------|:-------------|
/// | bid_fee |매수 수수료 비율 | NumberString |
/// | ask_fee| 매도 수수료 비율 | NumberString |
/// | maker_bid_fee| 매수 수수료 비율 | NumberString |
/// | maker_ask_fee| 매도 수수료 비율 | NumberString |
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
/// let order_status = api_exchange::get_order_status_by_uuid("9ca023a5-851b-4fec-9f0a-48cd83c2eaae").await;
/// ```
/// - parameters
/// > `uuid` uuid of order to cancel <br>
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
/// # Response Description
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

/// 주문 UUID 를 통해 개별 주문건을 조회한다. (inquire each order status via order UUID.)
///
/// # Example
/// ```
/// let order_status = api_exchange::get_order_status_by_identifier("test_identfier").await;
/// ```
/// - parameters
/// > `identifier` arbitrary identifier you want<br>
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
/// # Response Description
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
pub async fn get_order_status_by_identifier(
    identifier: &str,
) -> Result<OrderStatus, ResponseError> {
    OrderStatus::get_order_status_by_identifier(identifier).await
}

/// 주문 리스트를 조회한다. (inquire every order status.)
///
/// # Example
/// ```
/// let order_status = api_exchange::list_order_status().await;
/// ```
/// # Response
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
/// ]
/// ```
/// # Response Description
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
#[allow(deprecated)]
#[deprecated(since = "1.6.0", note = "use get_order_status_*() instead")]
pub async fn list_order_status() -> Result<Vec<OrderInfo>, ResponseError> {
    OrderInfo::get_order_state_list().await
}

/// 주문 리스트를 조회한다. (inquire every order status.)
///
/// # Example
/// ```
/// let order_status_list = api_exchange::get_order_status_by_uuids(
///     "KRW-ETH",
///     &["d60dfc8a-db0a-4087-9974-fed6433eb8f1"],
///     OrderBy::Desc
/// ).await;
/// ```
/// - parameters
/// > `market_id` ex) KRW-ETH<br>
/// > `uuids` arbitrary uuids you want<br>
/// > `order_by`
/// >> *  `OrderBy::Asc` 오름차순<br>
/// >> *  `OrderBy::Desc` 내림차순<br>
/// # Response
/// ```json
/// [
///   {
///     "uuid": "d60dfc8a-db0a-4087-9974-fed6433eb8f1",
///     "side": "ask",
///     "ord_type": "limit",
///     "price": "4280000.0",
///     "state": "done",
///     "market": "KRW-ETH",
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
/// ]
/// ```
/// # Response Description
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
pub async fn get_order_status_by_uuids(
    market_id: &str,
    uuids: &[&str],
    order_by: OrderBy,
) -> Result<Vec<OrderInfo>, ResponseError> {
    OrderInfo::get_order_status_by_uuids(market_id, uuids, order_by).await
}

/// 주문 리스트를 조회한다. (inquire every order status.)
///
/// # Example
/// ```
/// let order_status_list = api_exchange::get_order_status_by_identifiers(
///     "KRW-ETH",
///     &["test_identifier"],
///     OrderBy::Desc
/// ).await;
/// ```
/// - parameters
/// > `market_id` ex) KRW-ETH<br>
/// > `identifiers` arbitrary identifiers you want<br>
/// > `order_by`
/// >> *  `OrderBy::Asc` 오름차순<br>
/// >> *  `OrderBy::Desc` 내림차순<br>
/// # Response
/// ```json
/// [
///   {
///     "uuid": "d60dfc8a-db0a-4087-9974-fed6433eb8f1",
///     "side": "ask",
///     "ord_type": "limit",
///     "price": "4280000.0",
///     "state": "done",
///     "market": "KRW-ETH",
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
/// ]
/// ```
/// # Response Description
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
pub async fn get_order_status_by_identifiers(
    market_id: &str,
    identifiers: &[&str],
    order_by: OrderBy,
) -> Result<Vec<OrderInfo>, ResponseError> {
    OrderInfo::get_order_status_by_identifiers(market_id, identifiers, order_by).await
}

/// 주문 리스트를 조회한다. (inquire every order status.)
///
/// # Example
/// ```
/// let order_status_list = OrderInfo::request_get_orders_opened(
///     "KRW-ETH",
///     &[OrderState::Wait], // Only OrderState::Wait or OrderState::Watch have to be input
///     1,
///     10,
///     OrderBy::Desc,
/// ).await;
/// ```
/// - parameters
/// > `market_id` ex) KRW-ETH<br>
/// > `states` Array of OrderState
/// >> *  `OrderState::Wait` 대기<br>
/// >> *  `OrderState::Watch` 주문 중<br>
/// > `page` page number. 1~ <br>
/// > `limit` number of orders per page. 1~100<br>
/// > `order_by`
/// >> *  `OrderBy::Asc` 오름차순<br>
/// >> *  `OrderBy::Desc` 내림차순<br>
/// # Response
/// ```json
/// [
///   {
///     "uuid": "d60dfc8a-db0a-4087-9974-fed6433eb8f1",
///     "side": "ask",
///     "ord_type": "limit",
///     "price": "4280000.0",
///     "state": "done",
///     "market": "KRW-ETH",
///     "created_at": "2019-01-04T13:48:09+09:00",
///     "volume": "1.0",
///     "remaining_volume": "0.0",
///     "reserved_fee": "0.0",
///     "remaining_fee": "0.0",
///     "paid_fee": "2140.0",
///     "locked": "0.0",
///     "executed_volume": "1.0",
///     "executed_funds": null,
///     "trades_count": 1,
///   }
/// ]
/// ```
/// # Response Description
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
pub async fn get_order_status_opened(
    market_id: &str,
    states: &[OrderState],
    page: u8,
    limit: u8,
    order_by: OrderBy,
) -> Result<Vec<OrderInfo>, ResponseError> {
    OrderInfo::get_order_status_opened(market_id, states, page, limit, order_by).await
}

/// 주문 리스트를 조회한다. (inquire every order status.)
///
/// # Example
/// ```
/// let order_status_list = OrderInfo::request_get_orders_closed(
///     "KRW-ETH",
///     &[OrderState::Done], // Only OrderState::Done or OrderState::Cancel have to be input
///     None,
///     None,
///     10,
///     OrderBy::Desc,
/// ).await;
/// ```
/// - parameters
/// > `market_id` ex) KRW-ETH<br>
/// > `states` Array of OrderState
/// >> *  `OrderState::Done` 완료<br>
/// >> *  `OrderState::Cancel` 취소<br>
/// > `start_time` (optional) start time of the order<br>
/// > `end_time` (optional) end time of the order<br>
/// > `page` page number. 1~ <br>
/// > `limit` number of orders per page. 1~100<br>
/// > `order_by`
/// >> *  `OrderBy::Asc` 오름차순<br>
/// >> *  `OrderBy::Desc` 내림차순<br>
/// # Response
/// ```json
/// [
///   {
///     "uuid": "d60dfc8a-db0a-4087-9974-fed6433eb8f1",
///     "side": "ask",
///     "ord_type": "limit",
///     "price": "4280000.0",
///     "state": "done",
///     "market": "KRW-ETH",
///     "created_at": "2019-01-04T13:48:09+09:00",
///     "volume": "1.0",
///     "remaining_volume": "0.0",
///     "reserved_fee": "0.0",
///     "remaining_fee": "0.0",
///     "paid_fee": "2140.0",
///     "locked": "0.0",
///     "executed_volume": "1.0",
///     "executed_funds": null,
///     "trades_count": 1,
///   }
/// ]
/// ```
/// # Response Description
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
pub async fn get_order_status_closed(
    market_id: &str,
    states: &[OrderState],
    start_time: Option<&str>,
    end_time: Option<&str>,
    limit: u16,
    order_by: OrderBy,
) -> Result<Vec<OrderInfo>, ResponseError> {
    OrderInfo::get_order_status_closed(market_id, states, start_time, end_time, limit, order_by)
        .await
}

pub fn price_checker(price: f64) -> f64 {
    let truncation = if price >= 2_000_000.0 {
        1000.0
    } else if price >= 1_000_000.0 {
        500.0
    } else if price >= 500_000.0 {
        100.0
    } else if price >= 100_000.0 {
        50.0
    } else if price >= 10_000.0 {
        10.0
    } else if price >= 1000.0 {
        1.0
    } else if price >= 100.0 {
        0.1
    } else if price >= 10.0 {
        0.01
    } else if price >= 1.0 {
        0.001
    } else if price >= 0.1 {
        0.0001
    } else if price >= 0.01 {
        0.00001
    } else if price >= 0.001 {
        0.000001
    } else if price >= 0.0001 {
        0.0000001
    } else {
        0.00000001
    };

    f64::trunc(price / truncation) * truncation
}
