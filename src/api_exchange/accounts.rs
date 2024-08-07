use reqwest::header::{ACCEPT, AUTHORIZATION};
use reqwest::Response;

use crate::request::Request;

use super::{
    super::constant::{URL_ACCOUNTS, URL_SERVER},
    super::response::ResponseError,
    super::response::{AccountsInfo, AccountsInfoSource},
};

impl AccountsInfo {
    pub async fn get_account_info() -> Result<Vec<Self>, ResponseError> {
        let res = Self::request().await?;
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
            .map(|i: Vec<AccountsInfoSource>| {
                i.into_iter()
                    .map(|x| Self {
                        currency: x.currency(),
                        balance: x.balance(),
                        locked: x.locked(),
                        avg_buy_price: x.avg_buy_price(),
                        avg_buy_price_modified: x.avg_buy_price_modified(),
                        unit_currency: x.unit_currency(),
                    })
                    .collect::<Vec<Self>>()
            })
            .map_err(crate::response::response_error_from_json)
    }

    async fn request() -> Result<Response, ResponseError> {
        let token_string = Self::set_token()?;

        reqwest::Client::new()
            .get(format!("{URL_SERVER}{URL_ACCOUNTS}"))
            .header(ACCEPT, "application/json")
            .header(AUTHORIZATION, &token_string)
            .send()
            .await
            .map_err(crate::response::response_error_from_reqwest)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use serde_json::Value;

    use super::*;

    #[tokio::test]
    async fn test_get_account_info() {
        crate::set_access_key(&std::env::var("TEST_ACCESS_KEY").expect("TEST_ACCESS_KEY not set"));
        crate::set_secret_key(&std::env::var("TEST_SECRET_KEY").expect("TEST_SECRET_KEY not set"));

        let res = AccountsInfo::request().await.unwrap();
        let res_serialized = res
            .text()
            .await
            .map_err(crate::response::response_error_from_reqwest)
            .unwrap();

        if res_serialized.contains("error") {
            assert!(false, "Error response: {res_serialized}");
        }

        let json = serde_json::from_str::<Value>(&res_serialized).unwrap();
        let expected_keys = [
            "currency",
            "balance",
            "locked",
            "avg_buy_price",
            "avg_buy_price_modified",
            "unit_currency",
        ]
        .iter()
        .cloned()
        .collect::<HashSet<&str>>();

        if let Value::Array(json) = json {
            if json.len() == 0 {
                println!("[get_account_info] The JSON is empty.");
                assert!(true);
            }

            if let Value::Object(map) = &json[0] {
                let json_keys = map.keys().map(|k| k.as_str()).collect::<HashSet<&str>>();
                let unexpected_keys = json_keys.difference(&expected_keys).collect::<HashSet<_>>();
                let missing_keys = expected_keys.difference(&json_keys).collect::<HashSet<_>>();

                if !unexpected_keys.is_empty() {
                    println!("[get_account_info] Unexpected keys found: {unexpected_keys:?}");
                    assert!(false);
                } else {
                    println!("[get_account_info] No unexpected keys found.");
                }

                if !missing_keys.is_empty() {
                    println!("[get_account_info] Missing keys: {missing_keys:?}");
                    assert!(false);
                } else {
                    println!("[get_account_info] No keys are missing.");
                }
            } else {
                println!("[get_account_info] The JSON is not an object.");
            }
        }

        assert!(true);
    }
}
