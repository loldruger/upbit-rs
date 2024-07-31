use crate::request::Request;
use serde::{Deserialize, Serialize};

/// Deserialized and parsed account info data
#[derive(Serialize, Deserialize, Debug)]
pub struct AccountsInfo {
    pub currency: String,
    pub balance: f64,
    pub locked: f64,
    pub avg_buy_price: f64,
    pub avg_buy_price_modified: bool,
    pub unit_currency: String,
}

impl Request for AccountsInfo {}

/// Raw account info from serialized data
#[derive(Deserialize)]
pub struct AccountsInfoSource {
    currency: String,
    balance: String,
    locked: String,
    avg_buy_price: String,
    avg_buy_price_modified: bool,
    unit_currency: String,
}

impl AccountsInfoSource {
    /// Convert type of currency into [String]
    pub fn currency(&self) -> String { self.currency.to_owned() }
    /// Convert type of balance into [f64]
    pub fn balance(&self) -> f64 { self.balance.parse().unwrap() }
    /// Convert type of locked into [f64]
    pub fn locked(&self) -> f64 { self.locked.parse().unwrap() }
    /// Convert type of avg_buy_price into [f64]
    pub fn avg_buy_price(&self) -> f64 { self.avg_buy_price.parse().unwrap() }
    /// Convert type of avg_buy_price_modified into [bool]
    pub fn avg_buy_price_modified(&self) -> bool { self.avg_buy_price_modified }
    /// Convert type of unit_currency into [bool]
    pub fn unit_currency(&self) -> String { self.unit_currency.to_owned() }
}
