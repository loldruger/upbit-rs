use crate::response::ResponseError;

use super::super::constant::{URL_SERVER, URL_TRADES_TICKS};

use reqwest::{Url, Response};
use reqwest::header::ACCEPT;
use serde::Deserialize;

#[derive(Deserialize)]
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
    pub async fn list_trade_recent(market: &str, hhmmss: Option<&str>, count: i32, cursor: &str, days_ago: Option<i32>) -> Result<Self, ResponseError> {
        let res = Self::request(market, hhmmss, count, cursor, days_ago).await?;
        let res_serialized = res.text().await.map_err(crate::response::response_error_from_reqwest)?;
        
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
            .map_err(crate::response::response_error_from_json)
    }

    async fn request(market: &str, hhmmss: Option<&str>, count: i32, cursor: &str, days_ago: Option<i32>) -> Result<Response, ResponseError> {
        let mut url = Url::parse(&format!("{URL_SERVER}{URL_TRADES_TICKS}")).unwrap();
        url.query_pairs_mut()
            .append_pair("market", market)
            .append_pair("count", count.to_string().as_str())
            .append_pair("cursor", cursor);

        if let Some(hhmmss) = hhmmss {
            url.query_pairs_mut().append_pair("to", hhmmss);
        }

        if let Some(days_ago) = days_ago {
            url.query_pairs_mut().append_pair("daysAgo", days_ago.to_string().as_str());
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

    use serde_json::{Value, json};

    use crate::api_quotation::TradeRecent;

    #[tokio::test]
    async fn test_list_trade_recent() {
        crate::set_access_key(&std::env::var("TEST_ACCESS_KEY").expect("TEST_ACCESS_KEY not set"));
        crate::set_secret_key(&std::env::var("TEST_SECRET_KEY").expect("TEST_SECRET_KEY not set"));
    
        let res = TradeRecent::request("KRW-ETH", Some("120101"), 1, "1", None).await.unwrap();
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
            "trade_date_utc": "",
            "trade_time_utc": "",
            "timestamp": "",
            "trade_price": "",
            "trade_volume": "",
            "prev_closing_price": "",
            "change_price": "",
            "ask_bid": ""
        }]);

        let expected_structure = expected_structure[0]
        .as_object()
        .unwrap()
        .iter()
        .map(|(k, v)| (k.as_str(), v.clone()))
        .collect::<HashMap<&str, Value>>();

        if let Some(json_array) = json.as_array() {
            for (index, item) in json_array.iter().enumerate() {
                let (missing_keys, extra_keys) = compare_keys(item, &expected_structure, &format!("item[{}].", index));
    
                if !missing_keys.is_empty() {
                    println!("[test_list_trade_recent] Missing keys in item[{}]: {:?}", index, missing_keys);
                    assert!(false);
                } else {
                    println!("[test_list_trade_recent] No keys are missing in item[{}]", index);
                    assert!(true);
                }
    
                if !extra_keys.is_empty() {
                    println!("[test_list_trade_recent] Extra keys in item[{}]: {:?}", index, extra_keys);
                    assert!(false);
                } else {
                    println!("[test_list_trade_recent] No extra keys found in item[{}]", index);
                    assert!(true);
                }
            }
        } else {
            assert!(false, "Expected an array of objects in the response");
        }
    }

    fn compare_keys(json: &Value, expected: &HashMap<&str, Value>, path: &str) -> (Vec<String>, Vec<String>) {
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