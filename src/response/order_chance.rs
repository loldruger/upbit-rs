use super::accounts_info::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ObjectMarket {
    pub id: String,
    pub name: String,
    pub order_types: Vec<String>,
    pub order_sides: Vec<String>,
    pub bid: ObjectAskBid,
    pub ask: ObjectAskBid,
    pub max_total: u64,
    pub state: String,
}

#[derive(Deserialize, Debug)]
pub struct ObjectAskBid {
    pub currency: String,
    pub price_unit: Option<String>,
    pub min_total: u32,
}

#[derive(Deserialize, Debug)]
pub struct OrderChance {
    pub bid_fee: f32,
    pub ask_fee: f32,
    pub market: ObjectMarket,
    pub bid_account: AccountsInfo,
    pub ask_account: AccountsInfo,
}
