use crate::api::request::{Request, RequestWithQuery};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct OrderInfo {
    pub uuid: String,
    pub side: String,
    pub ord_type: String,
    pub price: f64,
    pub state: String,
    pub market: String,
    pub created_at: String,
    pub volume: f64,
    pub remaining_volume: f64,
    pub reserved_fee: f64,
    pub remaining_fee: f64,
    pub paid_fee: f64,
    pub locked: f64,
    pub executed_volume: f64,
    pub trades_count: i64,
}

impl Request for OrderInfo {}
impl RequestWithQuery for OrderInfo {}