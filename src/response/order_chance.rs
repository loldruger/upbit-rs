use serde::{Deserialize, Serialize};

use crate::api_exchange::{OrderType, OrderSide};

use super::accounts_info::*;

/// Deserialized ObjectMarket data of [OrderChance].
#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectMarket {
    pub id: String,
    pub name: String,
    // pub order_types: Vec<OrderType>,
    pub order_sides: Vec<OrderSide>,
    pub bid: ObjectAskBid,
    pub ask: ObjectAskBid,
    pub max_total: u64,
    pub state: String,
}

/// Deserialized ObjectAskBid info of [ObjectMarket].
#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectAskBid {
    pub currency: String,
    pub price_unit: Option<String>,
    pub min_total: u32,
}

/// Raw OrderChance data from serialized data
#[derive(Serialize, Deserialize, Debug)]
pub struct OrderChance {
    pub bid_fee: f32,
    pub ask_fee: f32,
    pub market: ObjectMarket,
    pub ask_types: Option<Vec<String>>,
    pub bid_types: Option<Vec<String>>,
    pub bid_account: AccountsInfo,
    pub ask_account: AccountsInfo,
}

/// Raw ObjectAskBidSource data of [OrderChanceSource]
#[derive(Deserialize)]
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

/// Raw ObjectAskBidSource data from serialized data
#[derive(Deserialize)]
pub struct ObjectAskBidSource {
    pub currency: String,
    pub price_unit: Option<String>,
    pub min_total: String,
}

/// Raw OrderChanceSource data from serialized data
#[derive(Deserialize)]
pub struct OrderChanceSource {
    pub bid_fee: String,
    pub ask_fee: String,
    pub market: ObjectMarketSource,
    pub ask_types: Option<Vec<String>>,
    pub bid_types: Option<Vec<String>>,
    pub bid_account: AccountsInfoSource,
    pub ask_account: AccountsInfoSource,
}