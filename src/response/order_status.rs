use crate::api_exchange::OrderSide;

use super::order_info::*;
use serde::{Deserialize, Serialize};

/// Deserialized ObjectTrades data of [OrderStatus].
#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectTrades {
    pub market: String,
    pub uuid: String,
    pub price: f64,
    pub volume: f64,
    pub funds: f64,
    pub side: OrderSide,

    #[cfg(feature = "chrono")]
    pub created_at: chrono::NaiveDateTime,
    #[cfg(not(any(feature = "chrono")))]
    pub created_at: String,
}

/// Deserialized OrderStatus data
#[derive(Serialize, Deserialize, Debug)]
pub struct OrderStatus {
    #[serde(flatten)]
    pub order_info: OrderInfo,
    pub trades: Vec<ObjectTrades>,
}

/// Raw ObjectTradesSource from serialized data
#[derive(Deserialize)]
pub struct ObjectTradesSource {
    pub market: String,
    pub uuid: String,
    pub price: String,
    pub volume: String,
    pub funds: String,
    pub side: String,
    pub created_at: String,
}

/// Raw OrderStatusSource from serialized data
#[derive(Deserialize)]
pub struct OrderStatusSource {
    #[serde(flatten)]
    pub order_info: OrderInfoSource,
    pub trades: Vec<ObjectTradesSource>,
}
