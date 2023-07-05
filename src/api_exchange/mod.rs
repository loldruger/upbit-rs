pub mod accounts;
pub mod request;

pub mod order;
pub mod order_cancel;
pub mod order_chance;
pub mod order_status;
pub mod order_status_list;

use crate::response_source::ResponseError;

use super::constant::{OrderSide, OrderType};
use super::response::{AccountsInfo, OrderInfo, OrderChance, OrderStatus};

/// 주문 요청을 한다.
/// 
///  Make an order(buy or sell) with desired price 
/// # Example
/// ```
/// let order_info = api::order_by_price("KRW-ETH", OrderSide::BID, 5000.0, 1_435_085.0, OrderType::LIMIT, None).await;
/// ```
/// # Response
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
    market: &str,
    side: OrderSide,
    price: f64,
    price_desired: f64,
    ord_type: OrderType,
    identifier: Option<&str>,
) -> Result<OrderInfo, ResponseError> {
    OrderInfo::order(
        market,
        side,
        Some((price + 1.) / price_checker(price_desired)),
        Some(price_checker(price_desired)),
        ord_type,
        identifier
    )
    .await
}

/// 즉시 시장가 판매를 한다.
/// 
/// Sell immediately at market price with specific amount of volume
/// # Example
/// ```
/// let order_info = api::sell_by_market_price("KRW-ETH", 1.0, "cdd92199-2897-4e14-9448-f923320408ad").await;
/// ```
/// # Response
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

pub async fn sell_by_market_price(market: &str, volume: f64, identifier: Option<&str>) -> Result<OrderInfo, ResponseError> {
    OrderInfo::order(
        market,
        OrderSide::ASK,
        Some(volume),
        None,
        OrderType::MARKET,
        identifier
    )
    .await
}

/// 주문을 취소한다.
/// 
/// Cancel an order
/// # Example
/// ```
/// let order_info = api::cancel_order("cdd92199-2897-4e14-9448-f923320408ad").await;
/// ```
/// # Response
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
pub async fn cancel_order(uuid: Option<&str>, identifier: Option<&str>) -> Result<OrderInfo, ResponseError> {
    OrderInfo::delete_order(uuid, identifier).await
}

/// 내가 보유한 자산 리스트를 보여줍니다.
/// 
/// inquire your account info
/// # Example
/// ```
/// let order_info = api::get_account_info().await;
/// ```
/// # Response
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

/// 마켓별 주문 가능 정보를 확인한다.
/// ```
/// let order_chance = api::get_order_chance("KRW-ETH").await;
/// ```
/// # Response
/// | field                  | description                   | type         |
/// |:-----------------------|:------------------------------|:-------------|
/// | bid_fee |매수 수수료 비율 | NumberString |
/// | ask_fee|	매도 수수료 비율 | NumberString |
/// | market|	마켓에 대한 정보 | Object |
/// | market.id|	마켓의 유일 키 | String |
/// | market.name|	마켓 이름 | String |
/// | market.order_types|	지원 주문 방식 (만료)	|Array[[String]] |
/// | ask_types|	매도 주문 지원 방식	|Array[[String]] |
/// | bid_types|	매수 주문 지원 방식	|Array[[String]] |
/// | market.order_sides|	지원 주문 종류	|Array[[String]] |
/// | market.bid|	매수 시 제약사항	| Object |
/// | market.bid.currency|	화폐를 의미하는 영문 대문자 코드	| String |
/// | market.bid.price_unit|	주문금액 단위	| String |
/// | market.bid.min_total|	최소 매도/매수 금액	| Number |
/// | market.ask|	매도 시 제약사항	| Object |
/// | market.ask.currency|	화폐를 의미하는 영문 대문자 코드	| String |
/// | market.ask.price_unit|	주문금액 단위	| String |
/// | market.ask.min_total|	최소 매도/매수 금액	| Number |
/// | market.max_total|	최대 매도/매수 금액	| NumberString |
/// | market.state|	마켓 운영 상태	| String |
/// | bid_account|	매수 시 사용하는 화폐의 계좌 상태	| Object |
/// | bid_account.currency|	화폐를 의미하는 영문 대문자 코드	| String |
/// | bid_account.balance|	주문가능 금액/수량	| NumberString |
/// | bid_account.locked|	주문 중 묶여있는 금액/수량	| NumberString |
/// | bid_account.avg_buy_price|	매수평균가	| NumberString |
/// | bid_account.avg_buy_price_modified|	매수평균가 수정 여부	| Boolean |
/// | bid_account.unit_currency|	평단가 기준 화폐	| String |
/// | ask_account|	매도 시 사용하는 화폐의 계좌 상태	| Object |
/// | ask_account.currency|	화폐를 의미하는 영문 대문자 코드	| String |
/// | ask_account.balance|	주문가능 금액/수량	| NumberString |
/// | ask_account.locked|	주문 중 묶여있는 금액/수량	| NumberString |
/// | ask_account.avg_buy_price|	매수평균가	| NumberString |
/// | ask_account.avg_buy_price_modified|	매수평균가 수정 여부	| Boolean |
/// | ask_account.unit_currency|	평단가 기준 화폐	| String |
pub async fn get_order_chance(market_id: &str) -> Result<OrderChance, ResponseError> {
    OrderChance::get_order_chance(market_id).await
}

/// 주문 UUID 를 통해 개별 주문건을 조회한다
/// 
/// inquire each order status via order UUID.
/// ```
/// let order_status = api::get_order_status("9ca023a5-851b-4fec-9f0a-48cd83c2eaae").await;
/// ```
/// # Response
/// | field                  | description                   | type         |
/// |:-----------------------|:------------------------------|:-------------|
/// | uuid | 주문의 고유 아이디	| String |
/// | side | 주문 종류	| String |
/// | ord_type | 주문 방식	| String |
/// | price | 주문 당시 화폐 가격	| NumberString |
/// | state | 주문 상태	| String |
/// | market | 마켓의 유일키	| String |
/// | created_at | 주문 생성 시간	| DateString |
/// | volume | 사용자가 입력한 주문 양	| NumberString |
/// | remaining_volume|체결 후 남은 주문 양	| NumberString |
/// | reserved_fee|수수료로 예약된 비용	| NumberString |
/// | remaining_fee|남은 수수료	| NumberString |
/// | paid_fee|사용된 수수료	| NumberString |
/// | locked|거래에 사용중인 비용	| NumberString |
/// | executed_volume|체결된 양	| NumberString |
/// | trades_count|해당 주문에 걸린 체결 수	| Integer |
/// | trades | 체결|Array[[Object]] |
/// | trades.market |마켓의 유일 키	| String |
/// | trades.uuid | 체결의 고유 아이디	| String |
/// | trades.price | 체결 가격	| NumberString |
/// | trades.volume | 체결 양	| NumberString |
/// | trades.funds | 체결된 총 가격	| NumberString |
/// | trades.side | 체결 종류	| String |
/// | trades.created_at | 체결 시각	| DateString |
pub async fn get_order_status(uuid: Option<&str>, identifier: Option<&str>) -> Result<OrderStatus, ResponseError> {
    OrderStatus::get_order_state(uuid, identifier).await
}

/// 주문 리스트를 조회한다.
/// 
/// inquire every order status
/// ```
/// let order_status = api::get_order_status_list().await;
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
pub async fn get_order_status_list() -> Result<Vec<OrderInfo>, ResponseError> {
    OrderInfo::get_order_state_list().await
}

fn price_checker(price: f64) -> f64 {
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
