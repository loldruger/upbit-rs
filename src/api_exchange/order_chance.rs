use reqwest::Response;
use reqwest::header::{ACCEPT, AUTHORIZATION};

use crate::request::RequestWithQuery;
use super::{
    super::constant::{URL_ORDER_CHANCE, URL_SERVER},
    super::response::{
        AccountsInfo,
        OrderChance,
        OrderChanceSource,
        ObjectMarket,
        ObjectAskBid,
        ResponseError
    },
};

impl RequestWithQuery for OrderChance {}
impl OrderChance {
    pub async fn get_order_chance(market_id: &str) -> Result<Self, ResponseError> {
        let res = Self::request(market_id).await?;
        let res_serialized = res.text().await.map_err(crate::response::response_error_from_reqwest)?;

        if res_serialized.contains("error") {
            return Err(serde_json::from_str(&res_serialized)
                .map(crate::response::response_error)
                .ok()
                .unwrap()
            )
        }

        serde_json::from_str(&res_serialized)
            .map(|x: OrderChanceSource| {
                Self {
                    bid_fee: x.bid_fee.parse().unwrap(),
                    ask_fee: x.ask_fee.parse().unwrap(),
                    market: ObjectMarket {
                        id: x.market.id.to_owned(),
                        name: x.market.name.to_owned(),
                        // order_types: x.market.order_types.into_iter().map(|x|x.as_str().into()).collect(),
                        order_sides: x.market.order_sides.into_iter().map(|x|x.as_str().into()).collect(),
                        bid: ObjectAskBid {
                            currency: x.market.bid.currency.to_owned(),
                            price_unit: x.market.bid.price_unit.to_owned(),
                            min_total: x.market.bid.min_total.parse().unwrap(),
                        },
                        ask: ObjectAskBid {
                            currency: x.market.ask.currency.to_owned(),
                            price_unit: x.market.ask.price_unit.to_owned(),
                            min_total: x.market.ask.min_total.parse().unwrap(),
                        },
                        max_total: x.market.max_total.parse().unwrap(),
                        state: x.market.state.to_owned(),
                    },
                    ask_types: None,
                    bid_types: None,
                    bid_account: AccountsInfo {
                        currency: x.bid_account.currency(),
                        balance: x.bid_account.balance(),
                        locked: x.bid_account.locked(),
                        avg_buy_price: x.bid_account.avg_buy_price(),
                        avg_buy_price_modified: x.bid_account.avg_buy_price_modified(),
                        unit_currency: x.bid_account.unit_currency(),
                    },
                    ask_account: AccountsInfo {
                        currency: x.ask_account.currency(),
                        balance: x.ask_account.balance(),
                        locked: x.ask_account.locked(),
                        avg_buy_price: x.ask_account.avg_buy_price(),
                        avg_buy_price_modified: x.ask_account.avg_buy_price_modified(),
                        unit_currency: x.ask_account.unit_currency(),
                    },
                }
            })
            .map_err(crate::response::response_error_from_json)
    }

    async fn request(market_id: &str) -> Result<Response, ResponseError> {
        let url = format!("{URL_SERVER}{URL_ORDER_CHANCE}/?market={market_id}");
        let token_string = Self::set_token_with_query(&url)?;

        reqwest::Client::new()
            .get(url)
            .header(ACCEPT, "application/json")
            .header(AUTHORIZATION, &token_string)
            .send()
            .await
            .map_err(crate::response::response_error_from_reqwest)
    }
}
