use super::{
    super::constant::URL_SERVER, super::response::ResponseError, CandleMinute, UrlAssociates,
};

use reqwest::header::ACCEPT;
use reqwest::{Response, Url};
use serde::Deserialize;

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
    pub unit: i64,
}

#[derive(Deserialize)]
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
    unit: i64,
}

impl CandleChartMinute {
    pub async fn request_candle(
        market_id: &str,
        to: Option<String>,
        count: i32,
        candle_minute: CandleMinute,
    ) -> Result<Vec<Self>, ResponseError> {
        let res = Self::request(market_id, to, count, candle_minute).await?;
        let res_serialized = res
            .text()
            .await
            .map_err(crate::response::response_error_from_reqwest)?;

        if res_serialized.contains("error") {
            return Err(serde_json::from_str(&res_serialized)
                .map(crate::response::response_error)
                .ok()
                .unwrap());
        }

        serde_json::from_str(&res_serialized)
            .map(|x: Vec<CandleChartMinuteSource>| {
                x.into_iter()
                    .map(|i| Self {
                        market: i.market,
                        candle_date_time_utc: chrono::NaiveDateTime::parse_from_str(
                            &i.candle_date_time_utc,
                            "%Y-%m-%dT%H:%M:%S",
                        )
                        .unwrap(),
                        candle_date_time_kst: chrono::NaiveDateTime::parse_from_str(
                            &i.candle_date_time_kst,
                            "%Y-%m-%dT%H:%M:%S",
                        )
                        .unwrap(),
                        opening_price: i.opening_price,
                        high_price: i.high_price,
                        low_price: i.low_price,
                        trade_price: i.trade_price,
                        timestamp: i.timestamp,
                        candle_acc_trade_price: i.candle_acc_trade_price,
                        candle_acc_trade_volume: i.candle_acc_trade_volume,
                        unit: i.unit,
                    })
                    .collect()
            })
            .map_err(crate::response::response_error_from_json)
    }

    async fn request(
        market_id: &str,
        to: Option<String>,
        count: i32,
        candle_minute: CandleMinute,
    ) -> Result<Response, ResponseError> {
        let url_candle = UrlAssociates::UrlCandleMinute(candle_minute).to_string();
        let mut url = Url::parse(&format!("{URL_SERVER}{url_candle}"))
            .map_err(crate::response::response_error_internal_url_parse_error)?;

        url.query_pairs_mut()
            .append_pair("market", market_id)
            .append_pair("count", count.to_string().as_str());

        if let Some(to) = to {
            url.query_pairs_mut().append_pair("to", to.as_str());
        }

        reqwest::Client::new()
            .get(url.as_str())
            .header(ACCEPT, "application/json")
            .send()
            .await
            .map_err(crate::response::response_error_from_reqwest)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json::Value;

    use crate::api_quotation::{CandleChartMinute, CandleMinute};

    #[tokio::test]
    async fn test_request_candle_minute() {
        crate::set_access_key(&std::env::var("TEST_ACCESS_KEY").expect("TEST_ACCESS_KEY not set"));
        crate::set_secret_key(&std::env::var("TEST_SECRET_KEY").expect("TEST_SECRET_KEY not set"));

        let res = CandleChartMinute::request("KRW-ETH", None, 1, CandleMinute::Min30)
            .await
            .unwrap();
        let res_serialized = res
            .text()
            .await
            .map_err(crate::response::response_error_from_reqwest)
            .unwrap();

        if res_serialized.contains("error") {
            assert!(false, "Error response: {res_serialized}");
        }

        let json = serde_json::from_str::<Value>(&res_serialized)
            .map_err(crate::response::response_error_from_json)
            .unwrap();
        let expected_structure = serde_json::json!([{
            "market": "",
            "candle_date_time_utc": "",
            "candle_date_time_kst": "",
            "opening_price": "",
            "high_price": "",
            "low_price": "",
            "trade_price": "",
            "timestamp": "",
            "candle_acc_trade_price": "",
            "candle_acc_trade_volume": "",
            "unit": ""
        }]);

        let expected_structure = expected_structure[0]
            .as_object()
            .unwrap()
            .iter()
            .map(|(k, v)| (k.as_str(), v.clone()))
            .collect::<HashMap<&str, Value>>();

        if let Some(json_array) = json.as_array() {
            for (index, item) in json_array.iter().enumerate() {
                let (missing_keys, extra_keys) =
                    compare_keys(item, &expected_structure, &format!("item[{}].", index));

                if !missing_keys.is_empty() {
                    println!(
                        "[test_request_candle_minute] Missing keys in item[{}]: {:?}",
                        index, missing_keys
                    );
                    assert!(false);
                } else {
                    println!(
                        "[test_request_candle_minute] No keys are missing in item[{}]",
                        index
                    );
                }

                if !extra_keys.is_empty() {
                    println!(
                        "[test_request_candle_minute] Extra keys in item[{}]: {:?}",
                        index, extra_keys
                    );
                    assert!(false);
                } else {
                    println!(
                        "[test_request_candle_minute] No extra keys found in item[{}]",
                        index
                    );
                }
            }
        } else {
            assert!(false, "Expected an array of objects in the response");
        }

        assert!(true);
    }

    fn compare_keys(
        json: &Value,
        expected: &HashMap<&str, Value>,
        path: &str,
    ) -> (Vec<String>, Vec<String>) {
        let mut missing_keys = Vec::new();
        let mut extra_keys = Vec::new();

        if let Some(actual_map) = json.as_object() {
            for (key, _) in expected {
                if !actual_map.contains_key(*key) {
                    missing_keys.push(format!("{}{}", path, key));
                }
            }
            for (key, _) in actual_map {
                if !expected.contains_key(key.as_str()) {
                    extra_keys.push(format!("{}{}", path, key));
                }
            }
        }

        (missing_keys, extra_keys)
    }
}
