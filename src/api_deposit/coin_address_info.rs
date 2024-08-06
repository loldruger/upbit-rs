use reqwest::header::{ACCEPT, AUTHORIZATION};
use reqwest::{Url, Response};

use crate::request::RequestWithQuery;

use super::{
    super::constant::{URL_DEPOSITS_COIN_ADDRESS, URL_SERVER},
    super::response::{
        CoinAddressResponse,
        ResponseError
    }
};

impl CoinAddressResponse {
    pub async fn get_coin_address_info(currency: &str, net_type: &str) -> Result<Self, ResponseError> {
        let res = Self::request(currency, net_type).await?;
        let res_serialized = res.text().await.map_err(crate::response::response_error_from_reqwest)?;

        if res_serialized.contains("error") {
            return Err(serde_json::from_str(&res_serialized)
                .map(crate::response::response_error)                
                .ok()
                .unwrap()
            )
        }
        
        serde_json::from_str(&res_serialized)
            .map(|x: CoinAddressResponse| {
                Self {
                    currency: x.currency,
                    net_type: x.net_type,
                    deposit_address: x.deposit_address,
                    secondary_address: x.secondary_address,
                }
            })
            .map_err(crate::response::response_error_from_json)
    }

    async fn request(currency: &str, net_type: &str) -> Result<Response, ResponseError> {
        let mut url = Url::parse(&format!("{URL_SERVER}{URL_DEPOSITS_COIN_ADDRESS}")).unwrap();

        url.query_pairs_mut()
            .append_pair("currency", currency)
            .append_pair("net_type", net_type);
        
        let token_string = Self::set_token_with_query(url.as_str())?;

        reqwest::Client::new()
            .get(url.as_str())
            .header(ACCEPT, "application/json")
            .header(AUTHORIZATION, &token_string)
            .send()
            .await
            .map_err(crate::response::response_error_from_reqwest)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use serde_json::Value;
    
    use crate::response::CoinAddressResponse;

    #[tokio::test]
    async fn test_get_coin_address_info() {
        crate::set_access_key(&std::env::var("TEST_ACCESS_KEY").expect("TEST_ACCESS_KEY not set"));
        crate::set_secret_key(&std::env::var("TEST_SECRET_KEY").expect("TEST_SECRET_KEY not set"));

        let res = CoinAddressResponse::request("KRW", "KRW").await.unwrap();
        let res_serialized = res.text().await.map_err(crate::response::response_error_from_reqwest).unwrap();
        
        if res_serialized.contains("error") {
            assert!(false, "Error response: {res_serialized}");
        }

        let json = serde_json::from_str::<Value>(&res_serialized).unwrap();
        let expected_structure = serde_json::json!({
            "currency": "",
            "net_type": "",
            "deposit_address": "",
            "secondary_address": "",
        });

        let expected_structure = expected_structure
            .as_object()
            .unwrap()
            .iter()
            .map(|(k, v)| (k.as_str(), v.clone()))
            .collect::<HashMap<&str, Value>>();
    
        let (missing_keys, extra_keys) = compare_keys(&json, &expected_structure, "");
    
        if !missing_keys.is_empty() {
            println!("[test_get_coin_address_info] Missing keys: {:?}", missing_keys);
            assert!(false);
        } else {
            println!("[test_get_coin_address_info] No keys are missing");
            assert!(true);
        }
    
        if !extra_keys.is_empty() {
            println!("[test_get_coin_address_info] Extra keys: {:?}", extra_keys);
            assert!(false);
        } else {
            println!("[test_get_coin_address_info] No extra keys found.");
            assert!(true);
        }
    }

    fn compare_keys(json: &Value, expected: &HashMap<&str, Value>, path: &str) -> (Vec<String>, Vec<String>) {
        let mut missing_keys = Vec::new();
        let mut extra_keys = Vec::new();
    
        if let Value::Object(map) = json {
            let json_keys: HashSet<&str> = map.keys().map(|k| k.as_str()).collect();
            let expected_keys: HashSet<&str> = expected.keys().cloned().collect();
    
            for key in expected_keys.difference(&json_keys) {
                missing_keys.push(format!("{}{}", path, key));
            }
    
            for key in json_keys.difference(&expected_keys) {
                extra_keys.push(format!("{}{}", path, key));
            }
    
            for key in expected_keys.intersection(&json_keys) {
                if let Some(expected_value) = expected.get(*key) {
                    let new_path = format!("{}{}.", path, key);
                    if let Value::Object(_) = expected_value {
                        let expected_map = expected_value.as_object().unwrap().iter().map(|(k, v)| (k.as_str(), v.clone())).collect::<HashMap<&str, Value>>();
                        let (mut missing, mut extra) = compare_keys(&map[*key], &expected_map, &new_path);
                        missing_keys.append(&mut missing);
                        extra_keys.append(&mut extra);
                    }
                }
            }
        }
    
        (missing_keys, extra_keys)
    }
}