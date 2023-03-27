use super::super::request::Request;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AccountsInfo {
    pub currency: String,
    pub balance: f64,
    pub locked: f64,
    pub avg_buy_price: f64,
    pub avg_buy_price_modified: bool,
    pub unit_currency: String,
}

#[derive(Deserialize, Debug)]
pub struct AccountsInfoSource {
    pub currency: String,
    pub balance: String,
    pub locked: String,
    pub avg_buy_price: String,
    pub avg_buy_price_modified: bool,
    pub unit_currency: String,
}

impl AccountsInfoSource {
    pub fn currency(&self) -> String { self.currency.to_owned() }
    pub fn balance(&self) -> f64 { self.balance.parse().unwrap() }
    pub fn locked(&self) -> f64 { self.locked.parse().unwrap() }
    pub fn avg_buy_price(&self) -> f64 { self.balance.parse().unwrap() }
    pub fn avg_buy_price_modified(&self) -> bool { self.avg_buy_price_modified }
    pub fn unit_currency(&self) -> String { self.unit_currency.to_owned() }
}

impl Request for AccountsInfo {}