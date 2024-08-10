use crate::response::ResponseError;

use super::super::constant::{URL_ORDERBOOK, URL_SERVER};

use reqwest::header::ACCEPT;
use reqwest::{Response, Url};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct OrderBookInfo {
    market: String,
    timestamp: i64,
    total_ask_size: f64,
    total_bid_size: f64,
    orderbook_units: Vec<OrderBookUnit>,
}

#[derive(Serialize, Deserialize)]
pub struct OrderBookUnit {
    ask_price: f64,
    bid_price: f64,
    ask_size: f64,
    bid_size: f64,
}

impl OrderBookInfo {
    pub async fn get_orderbook_info(markets_id: &[&str]) -> Result<Self, ResponseError> {
        let res = Self::request(markets_id).await?;
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
            .map(|mut i: Vec<Self>| {
                let x = i.pop().ok_or_else(|| crate::response::ResponseError {
                    state: crate::response::ResponseErrorState::CustomErrorNoDataPresent,
                    error: crate::response::ResponseErrorBody {
                        name: "custom_error_no_data_present".to_owned(),
                        message: "No data present in the response".to_owned(),
                    },
                })?;

                Ok(Self {
                    market: x.market,
                    timestamp: x.timestamp,
                    total_ask_size: x.total_ask_size,
                    total_bid_size: x.total_bid_size,
                    orderbook_units: x
                        .orderbook_units
                        .into_iter()
                        .map(|unit| OrderBookUnit {
                            ask_price: unit.ask_price,
                            bid_price: unit.bid_price,
                            ask_size: unit.ask_size,
                            bid_size: unit.bid_size,
                        })
                        .collect(),
                })
            })
            .map_err(crate::response::response_error_from_json)?
    }

    async fn request(markets_id: &[&str]) -> Result<Response, ResponseError> {
        let mut url = Url::parse(&format!("{URL_SERVER}{URL_ORDERBOOK}"))
            .map_err(crate::response::response_error_internal_url_parse_error)?;
        url.query_pairs_mut()
            .append_pair("markets", &markets_id.join(","));

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

    use crate::api_quotation::order_book::OrderBookInfo;

    #[tokio::test]
    async fn test_get_order_book() {
        crate::set_access_key(&std::env::var("TEST_ACCESS_KEY").expect("TEST_ACCESS_KEY not set"));
        crate::set_secret_key(&std::env::var("TEST_SECRET_KEY").expect("TEST_SECRET_KEY not set"));

        let res = OrderBookInfo::request(&["KRW-ETH"]).await.unwrap();
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
            "timestamp": "",
            "total_ask_size": "",
            "total_bid_size": "",
            "level": "",
            "orderbook_units": [
                {
                    "ask_price": "",
                    "bid_price": "",
                    "ask_size": "",
                    "bid_size": ""
                }
            ]
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
                        "[test_get_order_status_closed] Missing keys in item[{}]: {:?}",
                        index, missing_keys
                    );
                    assert!(false);
                } else {
                    println!(
                        "[test_get_order_status_closed] No keys are missing in item[{}]",
                        index
                    );
                }

                if !extra_keys.is_empty() {
                    println!(
                        "[test_get_order_status_closed] Extra keys in item[{}]: {:?}",
                        index, extra_keys
                    );
                    assert!(false);
                } else {
                    println!(
                        "[test_get_order_status_closed] No extra keys found in item[{}]",
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
