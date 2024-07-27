use crate::response::{ResponseError, ResponseErrorBody, ResponseErrorState, ResponseErrorSource};

use super::super::constant::{URL_SERVER, URL_TICKER};
use super::SnapshotChangeType;

use reqwest::Url;
use reqwest::header::ACCEPT;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct TickerSnapshot {
    market: String,
    trade_date: String,
    trade_time: String,
    trade_date_kst: String,
    trade_time_kst: String,
    trade_timestamp: i64,
    opening_price: f64,
    high_price: f64,
    low_price: f64,
    trade_price: f64,
    prev_closing_price: f64,
    change: SnapshotChangeType, //EVEN, RISE, FALL
    change_price: f64,
    change_rate: f64,
    signed_change_price: f64,
    signed_change_rate: f64,
    trade_volume: f64,
    acc_trade_price: f64,
    acc_trade_price_24h: f64,
    acc_trade_volume: f64,
    acc_trade_volume_24h: f64,
    highest_52_week_price: f64,
    highest_52_week_date: String,
    lowest_52_week_price: f64,
    lowest_52_week_date: String,
    timestamp: i64
}

impl TickerSnapshot {
    pub async fn get_ticker_snapshot(market: &str) -> Result<Self, ResponseError> {
        let res = Self::request(market).await?;
        let res_serialized = res.text().await.unwrap();
        
        if res_serialized.contains("error") {
            return Err(serde_json::from_str(&res_serialized)
                .map(crate::response::response_error)                
                .ok()
                .unwrap()
            )
        }
        
        serde_json::from_str(&res_serialized)
            .map(|mut x: Vec<Self>| {
                let x = x.pop().unwrap();
                Self {
                    market: x.market,
                    trade_date: x.trade_date,
                    trade_time: x.trade_time,
                    trade_date_kst: x.trade_date_kst,
                    trade_time_kst: x.trade_time_kst,
                    trade_timestamp: x.trade_timestamp,
                    opening_price: x.opening_price,
                    high_price: x.high_price,
                    low_price: x.low_price,
                    trade_price: x.trade_price,
                    prev_closing_price: x.prev_closing_price,
                    change: x.change,
                    change_price: x.change_price,
                    change_rate: x.change_rate,
                    signed_change_price: x.signed_change_price,
                    signed_change_rate: x.signed_change_rate,
                    trade_volume: x.trade_volume,
                    acc_trade_price: x.acc_trade_price,
                    acc_trade_price_24h: x.acc_trade_price_24h,
                    acc_trade_volume: x.acc_trade_volume,
                    acc_trade_volume_24h: x.acc_trade_volume_24h,
                    highest_52_week_price: x.highest_52_week_price,
                    highest_52_week_date: x.highest_52_week_date,
                    lowest_52_week_price: x.lowest_52_week_price,
                    lowest_52_week_date: x.lowest_52_week_date,
                    timestamp: x.timestamp,
                }
            })
            .map_err(crate::response::response_error_from_json)
    }

    async fn request(market: &str) -> Result<reqwest::Response, ResponseError> {
        let mut url = Url::parse(&format!("{URL_SERVER}{URL_TICKER}")).unwrap();
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