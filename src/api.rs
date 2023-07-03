pub mod accounts;
pub mod request;

pub mod order;
pub mod order_cancel;
pub mod order_chance;
pub mod order_state;
pub mod order_state_list;

use crate::response_source::ResponseError;

use super::constant::{OrderSide, OrderType};
use super::response::{AccountsInfo, OrderInfo, OrderChance, OrderState};

/// Make an order(buy or sell) with desired price 
/// # Example
/// ```
/// let order_info = api::order_by_price("KRW-ETH", OrderSide::BID, 5000.0, 1_435_085.0, OrderType::LIMIT, None).await;
/// ```
/// # Response
/// | field                  | description                   | type         |
/// |:-----------------------|:------------------------------|:-------------|
/// | currency               | 화폐를 의미하는 영문 대문자 코드 | String       |
/// | balance                | 주문가능 금액/수량              | NumberString |
/// | locked                 | 주문 중 묶여있는 금액/수량      | NumberString |
/// | avg_buy_price          | 매수평균가                     | NumberString |
/// | avg_buy_price_modified | 매수평균가 수정 여부            | Boolean     |
/// | unit_currency          | 평단가 기준 화폐                | String      |

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

pub async fn cancel_order(uuid: Option<&str>, identifier: Option<&str>) -> Result<OrderInfo, ResponseError> {
    OrderInfo::delete_order(uuid, identifier).await
}

pub async fn get_account_info() -> Result<Vec<AccountsInfo>, ResponseError> {
    AccountsInfo::get_account_info().await
}

pub async fn get_order_chance(market_id: &str) -> Result<OrderChance, ResponseError> {
    OrderChance::get_order_chance(market_id).await
}

pub async fn get_order_state(uuid: Option<&str>, identifier: Option<&str>) -> Result<OrderState, ResponseError> {
    OrderState::get_order_state(uuid, identifier).await
}

pub async fn get_order_state_list() -> Result<Vec<OrderInfo>, ResponseError> {
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
