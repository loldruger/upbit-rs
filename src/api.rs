pub mod accounts;
pub mod request;

pub mod order;
pub mod order_cancel;
pub mod order_chance;
pub mod order_state;
pub mod order_state_list;

use super::constant::{OrdSide, OrdType};
use super::response::{AccountsInfo, OrderInfo, OrderChance, OrderState, ResponseErrorState};

pub async fn order_by_price(
    market: &str,
    side: OrdSide,
    price: f64,
    price_desired: f64,
    ord_type: OrdType,
    identifier: Option<&str>,
) -> Result<OrderInfo, ResponseErrorState> {
    OrderInfo::order(
        market,
        side,
        Some((price + 1.) / price_checker(price_desired)),
        Some(price_checker(price_desired)),
        ord_type,
        identifier
    )
    .await
    .map_err(|e| ResponseErrorState::from(e.error.name.as_str()))
}

pub async fn sell_by_market_price(market: &str, volume: f64, identifier: Option<&str>) -> Result<OrderInfo, ResponseErrorState> {
    OrderInfo::order(
        market,
        OrdSide::ASK,
        Some(volume),
        None,
        OrdType::MARKET,
        identifier
    )
    .await
    .map_err(|e| ResponseErrorState::from(e.error.name.as_str()))
}

pub async fn cancel_order(uuid: Option<&str>, identifier: Option<&str>) -> Result<OrderInfo, ResponseErrorState> {
    OrderInfo::delete_order(uuid, identifier)
        .await
        .map_err(|e| ResponseErrorState::from(e.error.name.as_str()))
}

pub async fn get_account_info() -> Result<Vec<AccountsInfo>, ResponseErrorState> {
    AccountsInfo::get_account_info()
        .await
        .map_err(|e| ResponseErrorState::from(e.error.name.as_str()))
}

pub async fn get_order_chance(market_id: &str) -> Result<OrderChance, ResponseErrorState> {
    OrderChance::get_order_chance(market_id)
        .await
        .map_err(|e| ResponseErrorState::from(e.error.name.as_str()))
}

pub async fn get_order_state(uuid: Option<&str>, identifier: Option<&str>) -> Result<OrderState, ResponseErrorState> {
    OrderState::get_order_state(uuid, identifier)
        .await
        .map_err(|e| ResponseErrorState::from(e.error.name.as_str()))
}

pub async fn get_order_state_list() -> Result<Vec<OrderInfo>, ResponseErrorState> {
    OrderInfo::get_order_state_list()
        .await
        .map_err(|e| ResponseErrorState::from(e.error.name.as_str()))
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
