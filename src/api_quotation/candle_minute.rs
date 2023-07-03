use crate::response::ResponseErrorState;

use super::{
    super::constant::{URL_SERVER, UrlAssociates, CandleMinute},
    super::response_source::{ResponseError, ResponseErrorBody}
};

use reqwest::{Url, Response};
use reqwest::header::ACCEPT;
use serde::Deserialize;
use sqlx::types::chrono;

#[derive(Debug)]
pub struct CandleChartMinute {
    pub market: String,
    pub candle_date_time_utc: chrono::NaiveDateTime,
    pub candle_date_time_kst: chrono::NaiveDateTime,
    pub opening_price: f64,
    pub high_price: f64,
    pub low_price: f64,
    pub trade_price: f64,
    pub timestamp: i64,
    pub candle_acc_trade_price: f64,
    pub candle_acc_trade_volume: f64,
    pub unit: i64
}

#[derive(Deserialize, Debug)]
pub struct CandleChartMinuteSource {
    market: String,
    candle_date_time_utc: String,
    candle_date_time_kst: String,
    opening_price: f64,
    high_price: f64,
    low_price: f64,
    trade_price: f64,
    timestamp: i64,
    candle_acc_trade_price: f64,
    candle_acc_trade_volume: f64,
    unit: i64
}

impl CandleChartMinute {
    pub async fn request_candle(market: &str, to: Option<String>, count: i32, candle_minute: CandleMinute) -> Result<Vec<Self>, ResponseError> {
        let res = Self::request(market, to, count, candle_minute).await?;
        let res_serialized = res.text().await.unwrap();
        
        serde_json::from_str(&res_serialized)
        .map(|x: Vec<CandleChartMinuteSource>| {
            x
                .into_iter()
                .map(|i| {
                    Self {
                        market: i.market,
                        candle_date_time_utc: chrono::NaiveDateTime::parse_from_str(&i.candle_date_time_utc, "%Y-%m-%dT%H:%M:%S").unwrap(),
                        candle_date_time_kst: chrono::NaiveDateTime::parse_from_str(&i.candle_date_time_kst, "%Y-%m-%dT%H:%M:%S").unwrap(),
                        opening_price: i.opening_price,
                        high_price: i.high_price,
                        low_price: i.low_price,
                        trade_price: i.trade_price,
                        timestamp: i.timestamp,
                        candle_acc_trade_price: i.candle_acc_trade_price,
                        candle_acc_trade_volume: i.candle_acc_trade_volume,
                        unit: i.unit,
                    }
                })
                .collect()
        })
        .map_err(|x| {
            ResponseError {
                state: ResponseErrorState::InternalJsonParseError,
                error: ResponseErrorBody {
                    name: "internal_json_parse_error".to_owned(),
                    message: x.to_string()
                },
            }
        })
    }

    async fn request(market: &str, to: Option<String>, count: i32, candle_minute: CandleMinute) -> Result<Response, ResponseError> {
        let url_candle: String = UrlAssociates::UrlCandleMinute(candle_minute).into();
        let mut url = Url::parse(&format!("{URL_SERVER}{url_candle}")).unwrap();
        url.query_pairs_mut()
            .append_pair("market", market)
            .append_pair("count", count.to_string().as_str());
        
        if to.is_some() {
            url.query_pairs_mut().append_pair("to", to.unwrap().as_str());
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