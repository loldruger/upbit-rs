use reqwest::header::ACCEPT;
use reqwest::{Response, Url};
use serde::{Deserialize, Serialize};

use super::super::constant::{URL_MARKET_STATE, URL_SERVER};
use crate::response::ResponseError;

#[derive(Deserialize, Serialize, Debug)]
pub struct MarketState {
    market: String,
    korean_name: String,
    english_name: String,
    market_warning: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    market_event: Option<MarketEvent>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum MarketEvent {
    Warning(bool),
    Caution { caution: Caution, warning: bool },
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
pub struct Caution {
    CONCENTRATION_OF_SMALL_ACCOUNTS: bool,
    DEPOSIT_AMOUNT_SOARING: bool,
    GLOBAL_PRICE_DIFFERENCES: bool,
    PRICE_FLUCTUATIONS: bool,
    TRADING_VOLUME_SOARING: bool,
}

#[derive(Deserialize)]
pub struct MarketStateSource {
    market: String,
    korean_name: String,
    english_name: String,
    market_warning: String,
    market_event: MarketEvent,
}

impl MarketState {
    pub async fn get_market_state(is_detailed: bool) -> Result<Vec<Self>, ResponseError> {
        let res = Self::request(is_detailed).await?;
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

        serde_json::from_str::<Vec<MarketStateSource>>(&res_serialized)
            .map(|x| {
                x.into_iter()
                    .map(|i| Self {
                        market: i.market,
                        korean_name: i.korean_name,
                        english_name: i.english_name,
                        market_warning: Some(i.market_warning),
                        market_event: Some(i.market_event),
                    })
                    .collect()
            })
            .map_err(crate::response::response_error_from_json)
    }

    async fn request(is_detailed: bool) -> Result<Response, ResponseError> {
        let mut url = Url::parse(&format!("{URL_SERVER}{URL_MARKET_STATE}"))
            .map_err(crate::response::response_error_internal_url_parse_error)?;
        url.query_pairs_mut()
            .append_pair("isDetails", is_detailed.to_string().as_str());

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

    use crate::api_quotation::MarketState;

    #[tokio::test]
    async fn test_get_market_state() {
        crate::set_access_key(&std::env::var("TEST_ACCESS_KEY").expect("TEST_ACCESS_KEY not set"));
        crate::set_secret_key(&std::env::var("TEST_SECRET_KEY").expect("TEST_SECRET_KEY not set"));

        let res = MarketState::request(true).await.unwrap();
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
            "korean_name": "",
            "english_name": "",
            "market_warning": "",
            "market_event": {
                "warning": "",
                "caution": {
                    "CONCENTRATION_OF_SMALL_ACCOUNTS": "",
                    "DEPOSIT_AMOUNT_SOARING": "",
                    "GLOBAL_PRICE_DIFFERENCES": "",
                    "PRICE_FLUCTUATIONS": "",
                    "TRADING_VOLUME_SOARING": "",
                }
            }
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
                        "[test_get_market_state] Missing keys in item[{}]: {:?}",
                        index, missing_keys
                    );
                    assert!(false, "Missing keys found");
                }

                if !extra_keys.is_empty() {
                    println!(
                        "[test_get_market_state] Extra keys in item[{}]: {:?}",
                        index, extra_keys
                    );
                    assert!(false, "Extra keys found");
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

        if let Value::Object(actual_map) = json {
            for (key, expected_value) in expected {
                let current_path = if path.is_empty() {
                    key.to_string()
                } else {
                    format!("{}.{}", path, key)
                };

                match actual_map.get(*key) {
                    Some(actual_value) => {
                        // Check value types if the expected value is not an empty string placeholder
                        if !expected_value.is_string()
                            || !expected_value.as_str().unwrap().is_empty()
                        {
                            // Recurse if both values are objects
                            if expected_value.is_object() && actual_value.is_object() {
                                // Convert serde_json::Map to HashMap for the recursive call
                                let expected_object: HashMap<&str, Value> = expected_value
                                    .as_object()
                                    .unwrap()
                                    .iter()
                                    .map(|(k, v)| (k.as_str(), v.clone()))
                                    .collect();

                                let (mut missing, mut extra) =
                                    compare_keys(actual_value, &expected_object, &current_path);
                                missing_keys.append(&mut missing);
                                extra_keys.append(&mut extra);
                            } else if expected_value != actual_value {
                                // Compare if values are different
                                extra_keys.push(current_path);
                            }
                        }
                    }
                    None => missing_keys.push(current_path), // Key not found
                }
            }
            // Check for extra keys only if there are no missing keys.
            // This is to avoid false positives where extra keys are flagged due to missing required keys.
            if missing_keys.is_empty() {
                for key in actual_map.keys() {
                    if !expected.contains_key(key.as_str()) {
                        extra_keys.push(format!("{}.{}", path, key));
                    }
                }
            }
        }

        (missing_keys, extra_keys)
    }
}
