use crate::api::request::Request;
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

impl Request for AccountsInfo {}