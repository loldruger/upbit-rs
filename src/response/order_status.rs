use serde::Deserialize;
use super::order_info::*;

#[derive(Deserialize, Debug, Clone)]
pub struct ObjectTrades {
    pub market: String,
    pub uuid: String,
    pub price: f64,
    pub volume: f64,
    pub funds: f64,
    pub side: String,
    pub created_at: String,
}

#[derive(Deserialize, Debug)]
pub struct OrderStatus {
    #[serde(flatten)]
    pub order_info: OrderInfo,
    pub trades: Vec<ObjectTrades>,
}

