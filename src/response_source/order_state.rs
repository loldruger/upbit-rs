use serde::Deserialize;
use super::order_info::OrderInfoSource;

#[derive(Deserialize, Debug)]
pub struct ObjectTradesSource {
    pub market: String,
    pub uuid: String,
    pub price: String,
    pub volume: String,
    pub funds: String,
    pub side: String,
    pub created_at: String,
}

#[derive(Deserialize, Debug)]
pub struct OrderStateSource {
    #[serde(flatten)]
    pub order_info: OrderInfoSource,
    pub trades: Vec<ObjectTradesSource>,
}