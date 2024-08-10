use reqwest::header::{ACCEPT, AUTHORIZATION};
use reqwest::{Response, Url};

use crate::request::RequestWithQuery;
use crate::response::{TransactionInfo, TransactionInfoSource};

use super::{
    super::constant::{URL_DEPOSIT, URL_SERVER},
    super::response::ResponseError,
};

impl TransactionInfo {
    pub async fn get_deposit_info_by_currency(
        currency: &str,
    ) -> Result<Self, ResponseError> {
        let res = Self::request_deposit_by_currency(currency).await?;
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

        Self::deserialize_order_status_response(&res_serialized)
    }

    pub async fn get_deposit_info_by_uuid(
        uuid: &str,
    ) -> Result<Self, ResponseError> {
        let res = Self::request_deposit_by_uuid(uuid).await?;
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

        Self::deserialize_order_status_response(&res_serialized)
    }

    pub async fn get_deposit_info_by_txid(
        txid: &str,
    ) -> Result<Self, ResponseError> {
        let res = Self::request_deposit_by_txid(txid).await?;
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

        Self::deserialize_order_status_response(&res_serialized)
    }

    async fn request_deposit_by_currency(
        currency: &str
    ) -> Result<Response, ResponseError> {
        let mut url = Url::parse(&format!("{URL_SERVER}{URL_DEPOSIT}"))
            .map_err(crate::response::response_error_internal_url_parse_error)?;

        url.query_pairs_mut().append_pair("currency", currency);

        let token_string = Self::set_token_with_query(url.as_str())?;

        reqwest::Client::new()
            .get(url.as_str())
            .header(ACCEPT, "application/json")
            .header(AUTHORIZATION, &token_string)
            .send()
            .await
            .map_err(crate::response::response_error_from_reqwest)
    }

    async fn request_deposit_by_uuid(
        uuid: &str,
    ) -> Result<Response, ResponseError> {
        let mut url = Url::parse(&format!("{URL_SERVER}{URL_DEPOSIT}"))
            .map_err(crate::response::response_error_internal_url_parse_error)?;

        url.query_pairs_mut().append_pair("uuid", uuid);

        let token_string = Self::set_token_with_query(url.as_str())?;

        reqwest::Client::new()
            .get(url.as_str())
            .header(ACCEPT, "application/json")
            .header(AUTHORIZATION, &token_string)
            .send()
            .await
            .map_err(crate::response::response_error_from_reqwest)

    }

    async fn request_deposit_by_txid(
        txid: &str,
    ) -> Result<Response, ResponseError> {
        let mut url = Url::parse(&format!("{URL_SERVER}{URL_DEPOSIT}"))
            .map_err(crate::response::response_error_internal_url_parse_error)?;

        url.query_pairs_mut().append_pair("txid", txid);

        let token_string = Self::set_token_with_query(url.as_str())?;

        reqwest::Client::new()
            .get(url.as_str())
            .header(ACCEPT, "application/json")
            .header(AUTHORIZATION, &token_string)
            .send()
            .await
            .map_err(crate::response::response_error_from_reqwest)
    }

    fn deserialize_order_status_response(res_serialized: &str) -> Result<Self, ResponseError> {
        serde_json::from_str(res_serialized)
            .map(|x: TransactionInfoSource| Self {
                r#type: x.r#type(),
                uuid: x.uuid(),
                currency: x.currency(),
                net_type: x.net_type(),
                txid: x.txid(),
                state: x.state(),
                created_at: x.created_at(),
                done_at: x.done_at(),
                amount: x.amount(),
                fee: x.fee(),
                transaction_type: x.transaction_type(),

                holder: None,
                bank: None,
                fiat_amount: None,
                memo: None,
                fiat_currency: None,
                confirmations: None,
                krw_amount: None,
                network_name: None,
                cancelable: None,
                blockchain_url: None,
                state_i18n: None,
                address: None,
            })
            .map_err(crate::response::response_error_from_json)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use serde_json::Value;

    use crate::response::TransactionInfo;

    #[tokio::test]
    async fn test_get_deposit_info_by_currency() {
        crate::set_access_key(&std::env::var("TEST_ACCESS_KEY").expect("TEST_ACCESS_KEY not set"));
        crate::set_secret_key(&std::env::var("TEST_SECRET_KEY").expect("TEST_SECRET_KEY not set"));

        let res = TransactionInfo::request_deposit_by_currency("KRW")
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

        let json = serde_json::from_str::<Value>(&res_serialized).unwrap();
        let expected_structure = serde_json::json!({
            "type": "",
            "uuid": "",
            "currency": "",
            "net_type": "",
            "txid": "",
            "state": "",
            "created_at": "",
            "done_at": "",
            "amount": "",
            "fee": "",
            "transaction_type": ""
        });

        let expected_structure = expected_structure
            .as_object()
            .unwrap()
            .iter()
            .map(|(k, v)| (k.as_str(), v.clone()))
            .collect::<HashMap<&str, Value>>();

        let (missing_keys, extra_keys) = compare_keys(&json, &expected_structure, "");

        if !missing_keys.is_empty() {
            println!("[test_get_deposit_info] Missing keys: {missing_keys:?}");
            assert!(false);
        } else {
            println!("[test_get_deposit_info] No keys are missing");
        }

        if !extra_keys.is_empty() {
            println!("[test_get_deposit_info] Extra keys: {extra_keys:?}");
            assert!(false);
        } else {
            println!("[test_get_deposit_info] No extra keys found.");
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

        if let Value::Object(map) = json {
            let json_keys: HashSet<&str> = map.keys().map(|k| k.as_str()).collect();
            let expected_keys: HashSet<&str> = expected.keys().cloned().collect();

            for key in expected_keys.difference(&json_keys) {
                missing_keys.push(format!("{path}{key}"));
            }

            for key in json_keys.difference(&expected_keys) {
                extra_keys.push(format!("{path}{key}"));
            }

            for key in expected_keys.intersection(&json_keys) {
                if let Some(expected_value) = expected.get(*key) {
                    let new_path = format!("{}{}.", path, key);
                    if let Value::Object(_) = expected_value {
                        let expected_map = expected_value
                            .as_object()
                            .unwrap()
                            .iter()
                            .map(|(k, v)| (k.as_str(), v.clone()))
                            .collect::<HashMap<&str, Value>>();
                        let (mut missing, mut extra) =
                            compare_keys(&map[*key], &expected_map, &new_path);
                        missing_keys.append(&mut missing);
                        extra_keys.append(&mut extra);
                    }
                }
            }
        }

        (missing_keys, extra_keys)
    }
}
