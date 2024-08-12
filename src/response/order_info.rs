use crate::api_exchange::OrderSide;
use crate::{
    api_exchange::{OrderCondition, OrderState, OrderType},
    request::{Request, RequestWithQuery},
};
use serde::{Deserialize, Serialize};

/// Deserialized OrderInfo data
#[derive(Serialize, Deserialize, Debug)]
pub struct OrderInfo {
    pub uuid: String,
    pub side: OrderSide,
    pub ord_type: OrderType,
    pub price: Option<f64>,
    pub state: OrderState,
    pub market: String,
    
    #[cfg(feature = "chrono")]
    pub created_at: chrono::NaiveDateTime,
    #[cfg(not(any(feature = "chrono")))]
    pub created_at: String,

    pub volume: f64,
    pub remaining_volume: f64,
    pub reserved_fee: f64,
    pub remaining_fee: f64,
    pub paid_fee: f64,
    pub locked: f64,
    pub executed_volume: f64,
    pub executed_funds: Option<f64>,
    pub trades_count: i64,
    pub time_in_force: Option<OrderCondition>,
}

impl Request for OrderInfo {}
impl RequestWithQuery for OrderInfo {}

/// Raw OrderInfoSource info from serialized data
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
    executed_funds: Option<String>,
    trades_count: i64,
    time_in_force: Option<String>,
}

impl OrderInfoSource {
    /// Get uuid
    pub fn uuid(&self) -> String {
        self.uuid.to_owned()
    }
    /// Convert [String] type of side into [OrderSide]
    pub fn side(&self) -> OrderSide {
        self.side.as_str().into()
    }
    /// Convert [String] type of ord_type into [OrderType]
    pub fn ord_type(&self) -> OrderType {
        self.ord_type.as_str().into()
    }
    /// Convert [String] type of price into [f64]
    pub fn price(&self) -> Option<f64> {
        self.price.as_ref().and_then(|x| x.parse().ok())
    }
    /// Convert [String] type of state into [OrderState]
    pub fn state(&self) -> OrderState {
        self.state.as_str().into()
    }
    /// Get market
    pub fn market(&self) -> String {
        self.market.to_owned()
    }

    #[cfg(not(any(feature = "chrono")))]
    /// Convert [String] type of created_at into [String]
    pub fn created_at(&self) -> String {
        self.created_at.to_owned()
    }

    #[cfg(feature = "chrono")]
    /// Convert [String] type of volume into [chrono::NaiveDateTime]
    pub fn created_at(&self) -> chrono::NaiveDateTime {
        chrono::NaiveDateTime::parse_from_str(&self.created_at, "%Y-%m-%dT%H:%M:%S%z").unwrap()
    }

    /// Convert [String] type of volume into [f64]
    pub fn volume(&self) -> f64 {
        self.volume.parse().unwrap()
    }
    /// Convert [String] type of remaining_volume into [f64]
    pub fn remaining_volume(&self) -> f64 {
        self.remaining_volume.parse().unwrap()
    }
    /// Convert [String] type of reserved_fee into [f64]
    pub fn reserved_fee(&self) -> f64 {
        self.reserved_fee.parse().unwrap()
    }
    /// Convert [String] type of remaining_fee into [f64]
    pub fn remaining_fee(&self) -> f64 {
        self.remaining_fee.parse().unwrap()
    }
    /// Convert [String] type of paid_fee into [f64]
    pub fn paid_fee(&self) -> f64 {
        self.paid_fee.parse().unwrap()
    }
    /// Convert [String] type of locked into [f64]
    pub fn locked(&self) -> f64 {
        self.locked.parse().unwrap()
    }
    /// Convert [String] type of executed_volume into [f64]
    pub fn executed_volume(&self) -> f64 {
        self.executed_volume.parse().unwrap()
    }
    /// Convert [String] type of executed_funds into [f64]
    pub fn executed_funds(&self) -> Option<f64> {
        self.executed_funds.as_ref().and_then(|x| x.parse().ok())
    }
    /// Convert [String] type of trades_count into [f64]
    pub fn trades_count(&self) -> i64 {
        self.trades_count
    }
    /// Get time_in_force
    pub fn time_in_force(&self) -> Option<OrderCondition> {
        self.time_in_force
            .as_ref()
            .map(|x| OrderCondition::from(x.as_str()))
    }
}
