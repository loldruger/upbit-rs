use reqwest::header::{ACCEPT, AUTHORIZATION};
use reqwest::{Response, Url};

use crate::request::Request;

use super::{
    super::constant::{URL_DEPOSITS_COIN_ADDRESSES, URL_SERVER},
    super::response::{CoinAddressResponse, ResponseError},
};

impl CoinAddressResponse {
    pub async fn get_coin_address_info_list() -> Result<Vec<Self>, ResponseError> {
        let res = Self::request_list().await?;
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
            .map(|x: Vec<Self>| {
                x.into_iter()
                    .map(|x| Self {
                        currency: x.currency,
                        net_type: x.net_type,
                        deposit_address: x.deposit_address,
                        secondary_address: x.secondary_address,
                    })
                    .collect::<Vec<Self>>()
            })
            .map_err(crate::response::response_error_from_json)
    }

    async fn request_list() -> Result<Response, ResponseError> {
        let url = Url::parse(&format!("{URL_SERVER}{URL_DEPOSITS_COIN_ADDRESSES}"))
            .map_err(crate::response::response_error_internal_url_parse_error)?;
        let token_string = Self::set_token()?;

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
    use std::collections::HashMap;

    use serde_json::{json, Value};

    use crate::response::CoinAddressResponse;

    #[tokio::test]
    async fn test_get_deposit_list() {
        crate::set_access_key(&std::env::var("TEST_ACCESS_KEY").expect("TEST_ACCESS_KEY not set"));
        crate::set_secret_key(&std::env::var("TEST_SECRET_KEY").expect("TEST_SECRET_KEY not set"));

        let res = CoinAddressResponse::request_list().await.unwrap();
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
            "currency": "",
            "net_type": "",
            "deposit_address": "",
            "secondary_address": "",
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
                    compare_keys(item, &expected_structure, &format!("item[{index}]."));

                if !missing_keys.is_empty() {
                    println!(
                        "[test_get_deposit_list] Missing keys in item[{index}]: {missing_keys:?}"
                    );
                    assert!(false);
                } else {
                    println!("[test_get_deposit_list] No keys are missing in item[{index}]");
                }

                if !extra_keys.is_empty() {
                    println!("[test_get_deposit_list] Extra keys in item[{index}]: {extra_keys:?}");
                    assert!(false);
                } else {
                    println!("[test_get_deposit_list] No extra keys found in item[{index}]",);
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
                    missing_keys.push(format!("{path}{key}"));
                }
            }
            for (key, _) in actual_map {
                if !expected.contains_key(key.as_str()) {
                    extra_keys.push(format!("{path}{key}"));
                }
            }
        }

        (missing_keys, extra_keys)
    }
}
