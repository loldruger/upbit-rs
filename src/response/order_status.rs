use serde::Deserialize;
use super::order_info::*;

#[derive(Deserialize, Clone)]
pub struct ObjectTrades {
    pub market: String,
    pub uuid: String,
    pub price: f64,
    pub volume: f64,
    pub funds: f64,
    pub side: String,
    pub created_at: String,
}

#[derive(Deserialize)]
pub struct OrderStatus {
    #[serde(flatten)]
    pub order_info: OrderInfo,
    pub trades: Vec<ObjectTrades>,
}

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

#[derive(Deserialize)]
pub struct OrderStatusSource {
    #[serde(flatten)]
    pub order_info: OrderInfoSource,
    pub trades: Vec<ObjectTradesSource>,
}