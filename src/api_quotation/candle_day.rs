use crate::response::ResponseError;

use super::super::constant::URL_SERVER;
use super::UrlAssociates;

use reqwest::{Url, Response};
use reqwest::header::ACCEPT;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CandleChartDay {
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
    prev_closing_price: f64,
    change_price: f64,
    change_rate: f64,
    converted_trade_price: Option<f64>,
}

impl CandleChartDay {
    pub async fn request_candle(market: &str, count: i32, last_candle_time: Option<String>, price_unit: Option<String>) -> Result<Vec<Self>, ResponseError> {
        let res = Self::request(market, count, last_candle_time, price_unit).await?;
        let res_serialized = res.text().await.map_err(crate::response::response_error_from_reqwest)?;
        
        if res_serialized.contains("error") {
            return Err(serde_json::from_str(&res_serialized)
                .map(crate::response::response_error)                
                .ok()
                .unwrap()
            )
        }
        
        serde_json::from_str(&res_serialized)
            .map(|i: Vec<Self>| {         
                i
                    .into_iter()
                    .map(|x| {
                        Self {
                            market: x.market,
                            candle_date_time_utc: x.candle_date_time_utc,
                            candle_date_time_kst: x.candle_date_time_kst,
                            opening_price: x.opening_price,
                            high_price: x.high_price,
                            low_price: x.low_price,
                            trade_price: x.trade_price,
                            timestamp: x.timestamp,
                            candle_acc_trade_price: x.candle_acc_trade_price,
                            candle_acc_trade_volume: x.candle_acc_trade_volume,
                            prev_closing_price: x.prev_closing_price,
                            change_price: x.change_price,
                            change_rate: x.change_rate,
                            converted_trade_price: x.converted_trade_price
                        }
                    })
                    .collect()
        })
        .map_err(crate::response::response_error_from_json)
    }

    async fn request(market: &str, count: i32, last_candle_time: Option<String>, price_unit: Option<String>) -> Result<Response, ResponseError> {
        let url_candle = UrlAssociates::UrlCandleDay.to_string();
        let mut url = Url::parse(&format!("{URL_SERVER}{url_candle}")).unwrap();
        url.query_pairs_mut()
            .append_pair("market", market)
            .append_pair("count", count.to_string().as_str());

        if last_candle_time.is_some() {
            url.query_pairs_mut().append_pair("to", last_candle_time.unwrap().as_str());
        }

        if price_unit.is_some() {
            url.query_pairs_mut().append_pair("convertingPriceUnit", price_unit.unwrap().as_str());
        }
        
        reqwest::Client::new()
            .get(url.as_str())
            .header(ACCEPT, "application/json")
            .send()
            .await
            .map_err(crate::response::response_error_from_reqwest)
    }
}