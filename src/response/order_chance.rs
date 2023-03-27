use super::super::request::{Request, RequestWithQuery};
use super::accounts_info::*;
use serde::Deserialize;

//----------------Source----------------//
#[derive(Deserialize, Debug)]
pub struct ObjectMarketSource {
    pub id: String,
    pub name: String,
    pub order_types: Vec<String>,
    pub order_sides: Vec<String>,
    pub bid: ObjectAskBidSource,
    pub ask: ObjectAskBidSource,
    pub max_total: String,
    pub state: String,
}

#[derive(Deserialize, Debug)]
pub struct ObjectAskBidSource {
    pub currency: String,
    pub price_unit: Option<String>,
    pub min_total: String,
}
//------------------------------------//

//----------------Cast----------------//
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
//------------------------------------//

#[derive(Deserialize, Debug)]
pub struct OrderChance {
    pub bid_fee: f32,
    pub ask_fee: f32,
    pub market: ObjectMarket,
    pub bid_account: AccountsInfo,
    pub ask_account: AccountsInfo,
}

#[derive(Deserialize, Debug)]
pub struct OrderChanceSource {
    pub bid_fee: String,
    pub ask_fee: String,
    pub market: ObjectMarketSource,
    pub bid_account: AccountsInfoSource,
    pub ask_account: AccountsInfoSource,
}