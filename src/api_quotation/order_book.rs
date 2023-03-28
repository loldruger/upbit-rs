use crate::response_source::ResponseErrorSource;

use super::super::constant::{URL_SERVER, URL_ORDERBOOK};

use reqwest::{Url, Response};
use reqwest::header::ACCEPT;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct OrderbookInfo {
    market: String,
    timestamp: i64,
    total_ask_size: f64,
    total_bid_size: f64,
    orderbook_units: Vec<OrderBookUnit>
}

#[derive(Deserialize, Debug)]
pub struct OrderBookUnit {
    ask_price: f64,
    bid_price: f64,
    ask_size: f64,
    bid_size: f64
}

impl OrderbookInfo {
    pub async fn get_orderbook_info(market: &str) -> Result<Self, ResponseErrorSource> {       
        let res = Self::request(market).await; 
        let res_serialized = res.text().await.unwrap();
        let res_deserialized = serde_json::from_str(&res_serialized)
            .and_then(|mut x: Vec<Self>| {
                let x = x.pop().unwrap();
                let res = Self {
                    market: x.market,
                    timestamp: x.timestamp,
                    total_ask_size: x.total_ask_size,
                    total_bid_size: x.total_bid_size,
                    orderbook_units: x.orderbook_units
                        .into_iter()
                        .map(|unit| OrderBookUnit {
                            ask_price: unit.ask_price,
                            bid_price: unit.bid_price,
                            ask_size: unit.ask_size,
                            bid_size: unit.bid_size
                        })
                        .collect(),
                };
                Ok(res)
            })
            .map_err(|_| {
                let res_deserialized_error: ResponseErrorSource = serde_json::from_str(&res_serialized)
                    .and_then(|e: ResponseErrorSource| {
                        Ok(e)
                    })
                    .unwrap();

                res_deserialized_error
            });

        res_deserialized
    }

    async fn request(market: &str) -> Response {
        let mut url = Url::parse(&format!("{URL_SERVER}{URL_ORDERBOOK}")).unwrap();
        url.query_pairs_mut().append_pair("markets", market);

        let res = reqwest::Client::new()
            .get(url.as_str())
            .header(ACCEPT, "application/json")
            .send()
            .await
            .unwrap();

        res
    }
}