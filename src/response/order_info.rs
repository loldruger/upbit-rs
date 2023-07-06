use crate::request::{Request, RequestWithQuery};
use serde::Deserialize;

#[derive(Deserialize)]
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

#[derive(Deserialize)]
pub struct OrderInfoSource {
    uuid: String,
    side: String,
    ord_type: String,
    price: Option<String>,
    state: String,
    market: String,
    created_at: String,
    volume: String,
    remaining_volume: String,
    reserved_fee: String,
    remaining_fee: String,
    paid_fee: String,
    locked: String,
    executed_volume: String,
    trades_count: i64,
}

impl OrderInfoSource {
    pub fn uuid(&self) -> String { self.uuid.to_owned() }
    pub fn side(&self) -> String { self.side.to_owned() }
    pub fn ord_type(&self) -> String { self.ord_type.to_owned() }
    pub fn price(&self) -> f64 { 
        self.price
            .as_ref()
            .unwrap()
            .parse()
            .unwrap()
        }
    pub fn state(&self) -> String { self.state.to_owned() }
    pub fn market(&self) -> String { self.market.to_owned() }
    pub fn created_at(&self) -> String { self.created_at.to_owned() }
    pub fn volume(&self) -> f64 { self.volume.parse().unwrap() }
    pub fn remaining_volume(&self) -> f64 { self.remaining_volume.parse().unwrap() }
    pub fn reserved_fee(&self) -> f64 { self.reserved_fee.parse().unwrap() }
    pub fn remaining_fee(&self) -> f64 { self.remaining_fee.parse().unwrap() }
    pub fn paid_fee(&self) -> f64 { self.paid_fee.parse().unwrap() }
    pub fn locked(&self) -> f64 { self.locked.parse().unwrap() }
    pub fn executed_volume(&self) -> f64 { self.executed_volume.parse().unwrap() }
    pub fn trades_count(&self) -> i64 { self.trades_count }
}