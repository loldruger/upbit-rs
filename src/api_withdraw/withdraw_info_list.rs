use std::str::FromStr;

use crate::request::RequestWithQuery;
use reqwest::{
    header::{ACCEPT, AUTHORIZATION},
    Response, Url,
};

use super::{
    super::constant::{OrderBy, URL_SERVER, URL_WITHDRAWS},
    super::response::{ResponseError, TransactionInfo, TransactionInfoSource},
    WithdrawState,
};

impl TransactionInfo {
    pub async fn get_withdraw_list(
        currency: &str,
        state: WithdrawState,
        uuids: Option<&[&str]>,
        txids: Option<&[&str]>,
        limit: u32,
        page: u32,
        order_by: OrderBy,
    ) -> Result<Vec<Self>, ResponseError> {
        let res = Self::request(currency, state, uuids, txids, limit, page, order_by).await?;
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
            .map(|x: Vec<TransactionInfoSource>| {
                x.into_iter()
                    .map(|x| Self {
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
                    .collect::<Vec<Self>>()
            })
            .map_err(crate::response::response_error_from_json)
    }

    async fn request(
        currency: &str,
        state: WithdrawState,
        uuids: Option<&[&str]>,
        txids: Option<&[&str]>,
        limit: u32,
        page: u32,
        order_by: OrderBy,
    ) -> Result<Response, ResponseError> {
        let mut url = Url::parse(&format!("{URL_SERVER}{URL_WITHDRAWS}"))
            .map_err(crate::response::response_error_internal_url_parse_error)?;

        url.query_pairs_mut()
            .append_pair("currency", currency)
            .append_pair("state", &state.to_string())
            .append_pair("limit", &format!("{limit}"))
            .append_pair("page", &format!("{page}"))
            .append_pair("order_by", &order_by.to_string());

        let mut url_modified = if let Some(uuids) = uuids {
            for uuid in uuids {
                url.query_pairs_mut().append_pair("uuids", uuid);
            }

            Url::from_str(url.as_str().replace("uuids", "uuids[]").as_str())
                .map_err(crate::response::response_error_internal_url_parse_error)?
        } else {
            url
        };

        let url_modified = if let Some(txids) = txids {
            for txid in txids {
                url_modified.query_pairs_mut().append_pair("txids", txid);
            }

            url_modified.as_str().replace("txids", "txids[]")
        } else {
            url_modified.as_str().to_string()
        };

        let token_string = Self::set_token_with_query(&url_modified)?;

        reqwest::Client::new()
            .get(url_modified)
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

    use crate::{api_withdraw::WithdrawState, constant::OrderBy, response::TransactionInfo};

    #[tokio::test]
    async fn test_get_withdraw_list() {
        crate::set_access_key(&std::env::var("TEST_ACCESS_KEY").expect("TEST_ACCESS_KEY not set"));
        crate::set_secret_key(&std::env::var("TEST_SECRET_KEY").expect("TEST_SECRET_KEY not set"));

        let res = TransactionInfo::request(
            "ETH",
            WithdrawState::Waiting,
            None,
            None,
            1,
            1,
            OrderBy::Desc,
        )
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
        let expected_structure = json!([{
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
                        "[test_get_withdraw_list] Missing keys in item[{}]: {:?}",
                        index, missing_keys
                    );
                    assert!(false);
                } else {
                    println!(
                        "[test_get_withdraw_list] No keys are missing in item[{}]",
                        index
                    );
                    assert!(true);
                }

                if !extra_keys.is_empty() {
                    println!(
                        "[test_get_withdraw_list] Extra keys in item[{}]: {:?}",
                        index, extra_keys
                    );
                    assert!(false);
                } else {
                    println!(
                        "[test_get_withdraw_list] No extra keys found in item[{}]",
                        index
                    );
                    assert!(true);
                }
            }
        } else {
            assert!(false, "Expected an array of objects in the response");
        }
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
