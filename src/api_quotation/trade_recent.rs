use crate::response::ResponseErrorState;
use crate::response_source::{ResponseError, ResponseErrorBody};

use super::super::constant::{URL_SERVER, URL_TRADES_TICKS};

use reqwest::{Url, Response};
use reqwest::header::ACCEPT;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TradeRecent {
    market: String,
    trade_date_utc: String,
    trade_time_utc: String,
    timestamp: i64,
    trade_price: f64,
    trade_volume: f64,
    prev_closing_price: f64,
    chane_price: f64,
    ask_bid: String
}

impl TradeRecent {
    pub async fn get_trade_recent(market: &str, hhmmss: Option<&str>, count: i32, cursor: String, days_ago: Option<i32>) -> Result<Self, ResponseError> {
        let res = Self::request(market, hhmmss, count, cursor, days_ago).await?;
        let res_serialized = res.text().await.unwrap();
        
        serde_json::from_str(&res_serialized)
            .map(|mut x: Vec<Self>| {
                let x = x.pop().unwrap();

                Self {
                    market: x.market,
                    trade_date_utc: x.trade_date_utc,
                    trade_time_utc: x.trade_time_utc,
                    timestamp: x.timestamp,
                    trade_price: x.trade_price,
                    trade_volume: x.trade_volume,
                    prev_closing_price: x.prev_closing_price,
                    chane_price: x.chane_price,
                    ask_bid: x.ask_bid,
                }
            })
            .map_err(|_| serde_json::from_str(&res_serialized).unwrap())
    }

    async fn request(market: &str, hhmmss: Option<&str>, count: i32, cursor: String, days_ago: Option<i32>) -> Result<Response, ResponseError> {
        let mut url = Url::parse(&format!("{URL_SERVER}{URL_TRADES_TICKS}")).unwrap();
        url.query_pairs_mut()
            .append_pair("market", market)
            .append_pair("count", count.to_string().as_str())
            .append_pair("cursor", cursor.as_str());

        if hhmmss.is_some() {
            url.query_pairs_mut().append_pair("to", hhmmss.unwrap());
        }

        if days_ago.is_some() {
            url.query_pairs_mut().append_pair("daysAgo", days_ago.unwrap().to_string().as_str());
        }
        
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