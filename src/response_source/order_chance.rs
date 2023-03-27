use serde::Deserialize;
use super::accounts_info::AccountsInfoSource;

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

#[derive(Deserialize, Debug)]
pub struct OrderChanceSource {
    pub bid_fee: String,
    pub ask_fee: String,
    pub market: ObjectMarketSource,
    pub bid_account: AccountsInfoSource,
    pub ask_account: AccountsInfoSource,
}