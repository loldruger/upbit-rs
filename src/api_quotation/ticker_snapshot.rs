use crate::response::ResponseError;

use super::super::constant::{URL_SERVER, URL_TICKER};
use super::SnapshotChangeType;

use reqwest::header::ACCEPT;
use reqwest::Url;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TickerSnapshot {
    pub market: String,
    pub trade_date: String,
    pub trade_time: String,
    pub trade_date_kst: String,
    pub trade_time_kst: String,
    pub trade_timestamp: i64,
    pub opening_price: f64,
    pub high_price: f64,
    pub low_price: f64,
    pub trade_price: f64,
    pub prev_closing_price: f64,
    pub change: SnapshotChangeType, //EVEN, RISE, FALL
    pub change_price: f64,
    pub change_rate: f64,
    pub signed_change_price: f64,
    pub signed_change_rate: f64,
    pub trade_volume: f64,
    pub acc_trade_price: f64,
    pub acc_trade_price_24h: f64,
    pub acc_trade_volume: f64,
    pub acc_trade_volume_24h: f64,
    pub highest_52_week_price: f64,
    pub highest_52_week_date: String,
    pub lowest_52_week_price: f64,
    pub lowest_52_week_date: String,
    pub timestamp: i64,
}

#[derive(Deserialize, Debug)]
pub struct TickerSnapshotSource {
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
    change: String, //EVEN, RISE, FALL
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
    timestamp: i64,
}

impl TickerSnapshot {
    pub async fn get_ticker_snapshot(market: &[&str]) -> Result<Self, ResponseError> {
        let res = Self::request(market).await?;
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
            .map(|mut i: Vec<TickerSnapshotSource>| {
                let x = i.pop().ok_or_else(|| crate::response::ResponseError {
                    state: crate::response::ResponseErrorState::CustomErrorNoDataPresent,
                    error: crate::response::ResponseErrorBody {
                        name: "custom_error_no_data_present".to_owned(),
                        message: "No data present in the response".to_owned(),
                    },
                })?;

                Ok(Self {
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
                    change: x.change.as_str().into(),
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
                })
            })
            .map_err(crate::response::response_error_from_json)?
    }

    async fn request(market: &[&str]) -> Result<reqwest::Response, ResponseError> {
        let mut url = Url::parse(&format!("{URL_SERVER}{URL_TICKER}"))
            .map_err(crate::response::response_error_internal_url_parse_error)?;
        url.query_pairs_mut()
            .append_pair("markets", &market.join(","));

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

    use serde_json::{json, Value};

    use super::*;

    #[tokio::test]
    async fn test_get_ticker_snapshot() {
        crate::set_access_key(&std::env::var("TEST_ACCESS_KEY").expect("TEST_ACCESS_KEY not set"));
        crate::set_secret_key(&std::env::var("TEST_SECRET_KEY").expect("TEST_SECRET_KEY not set"));

        let res = TickerSnapshot::request(&["KRW-ETH"]).await.unwrap();
        let res_serialized = res
            .text()
            .await
            .map_err(crate::response::response_error_from_reqwest)
            .unwrap();

        if res_serialized.contains("error") {
            assert!(false, "Error response: {res_serialized}");
        }

        let json = serde_json::from_str::<Value>(&res_serialized).unwrap();
        let expected_structure = json!([{
            "market": "",
            "trade_date": "",
            "trade_time": "",
            "trade_date_kst": "",
            "trade_time_kst": "",
            "trade_timestamp": "",
            "opening_price": "",
            "high_price": "",
            "low_price": "",
            "trade_price": "",
            "prev_closing_price": "",
            "change": "",
            "change_price": "",
            "change_rate": "",
            "signed_change_price": "",
            "signed_change_rate": "",
            "trade_volume": "",
            "acc_trade_price": "",
            "acc_trade_price_24h": "",
            "acc_trade_volume": "",
            "acc_trade_volume_24h": "",
            "highest_52_week_price": "",
            "highest_52_week_date": "",
            "lowest_52_week_price": "",
            "lowest_52_week_date": "",
            "timestamp": ""
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
                        "[test_get_ticker_snapshot] Missing keys in item[{}]: {:?}",
                        index, missing_keys
                    );
                    assert!(false);
                } else {
                    println!(
                        "[test_get_ticker_snapshot] No keys are missing in item[{}]",
                        index
                    );
                }

                if !extra_keys.is_empty() {
                    println!(
                        "[test_get_ticker_snapshot] Extra keys in item[{}]: {:?}",
                        index, extra_keys
                    );
                    assert!(false);
                } else {
                    println!(
                        "[test_get_ticker_snapshot] No extra keys found in item[{}]",
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
