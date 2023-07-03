use crate::response::ResponseErrorState;
use crate::response_source::{ResponseError, ResponseErrorBody};

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
    pub async fn get_orderbook_info(market: &str) -> Result<Self, ResponseError> {       
        let res = Self::request(market).await?; 
        let res_serialized = res.text().await.unwrap();
        
        serde_json::from_str(&res_serialized)
            .map(|mut x: Vec<Self>| {
                let x = x.pop().unwrap();
                Self {
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
                }
            })
            .map_err(|_| serde_json::from_str(&res_serialized).unwrap())
    }

    async fn request(market: &str) -> Result<Response, ResponseError> {
        let mut url = Url::parse(&format!("{URL_SERVER}{URL_ORDERBOOK}")).unwrap();
        url.query_pairs_mut().append_pair("markets", market);

        reqwest::Client::new()
            .get(url.as_str())
            .header(ACCEPT, "application/json")
            .send()
            .await
            .map_err(|x| {
                ResponseError {
                    state: ResponseErrorState::InternalReqwestError,
                    error: ResponseErrorBody {
                        name: "internal_reqwest_error".to_owned(),
                        message: x.to_string()
                    }
                }
            })
    }
}