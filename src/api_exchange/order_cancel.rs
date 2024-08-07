use crate::request::RequestWithQuery;
use reqwest::{
    header::{ACCEPT, AUTHORIZATION},
    Response, Url,
};

use super::{
    super::constant::{URL_ORDER_STATUS, URL_SERVER},
    super::response::{OrderInfo, OrderInfoSource, ResponseError},
};

impl OrderInfo {
    pub async fn cancel_order_by_uuid(uuid: &str) -> Result<Self, ResponseError> {
        let res = Self::request_cancel_by_uuid(uuid).await?;
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

        Self::deserialize_order_cancel(&res_serialized)
    }

    pub async fn cancel_order_by_identifier(identifier: &str) -> Result<Self, ResponseError> {
        let res = Self::request_cancel_by_identifier(identifier).await?;
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

        Self::deserialize_order_cancel(&res_serialized)
    }

    async fn request_cancel_by_uuid(uuid: &str) -> Result<Response, ResponseError> {
        let mut url = Url::parse(&format!("{URL_SERVER}{URL_ORDER_STATUS}"))
            .map_err(crate::response::response_error_internal_url_parse_error)?;
        url.query_pairs_mut().append_pair("uuid", uuid);

        let token_string = Self::set_token_with_query(url.as_str())?;

        reqwest::Client::new()
            .delete(url.as_str())
            .header(ACCEPT, "application/json")
            .header(AUTHORIZATION, &token_string)
            .send()
            .await
            .map_err(crate::response::response_error_from_reqwest)
    }

    async fn request_cancel_by_identifier(identifier: &str) -> Result<Response, ResponseError> {
        let mut url = Url::parse(&format!("{URL_SERVER}{URL_ORDER_STATUS}"))
            .map_err(crate::response::response_error_internal_url_parse_error)?;
        url.query_pairs_mut().append_pair("identifier", identifier);

        let token_string = Self::set_token_with_query(url.as_str())?;

        reqwest::Client::new()
            .delete(url.as_str())
            .header(ACCEPT, "application/json")
            .header(AUTHORIZATION, &token_string)
            .send()
            .await
            .map_err(crate::response::response_error_from_reqwest)
    }

    fn deserialize_order_cancel(res_serialized: &str) -> Result<Self, ResponseError> {
        serde_json::from_str(res_serialized)
            .map(|x: OrderInfoSource| Self {
                uuid: x.uuid(),
                side: x.side(),
                ord_type: x.ord_type(),
                price: x.price(),
                state: x.state(),
                market: x.market(),
                created_at: x.created_at(),
                volume: x.volume(),
                remaining_volume: x.remaining_volume(),
                reserved_fee: x.reserved_fee(),
                remaining_fee: x.remaining_fee(),
                paid_fee: x.paid_fee(),
                locked: x.locked(),
                executed_volume: x.executed_volume(),
                executed_funds: x.executed_funds(),
                trades_count: x.trades_count(),
                time_in_force: x.time_in_force(),
            })
            .map_err(crate::response::response_error_from_json)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use serde_json::Value;

    use crate::{
        api_exchange::{OrderSide, OrderType},
        response::OrderInfo,
    };

    #[tokio::test]
    async fn test_order_cancel_by_uuid() {
        crate::set_access_key(&std::env::var("TEST_ACCESS_KEY").expect("TEST_ACCESS_KEY not set"));
        crate::set_secret_key(&std::env::var("TEST_SECRET_KEY").expect("TEST_SECRET_KEY not set"));

        let uuid = order_to_get_uuid().await;

        let res = OrderInfo::request_cancel_by_uuid(&uuid).await.unwrap();
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
            "uuid": "",
            "side": "",
            "ord_type": "",
            "price": "",
            "state": "",
            "market": "",
            "created_at": "",
            "volume": "",
            "remaining_volume": "",
            "reserved_fee": "",
            "remaining_fee": "",
            "paid_fee": "",
            "locked": "",
            "executed_volume": "",
            // "executed_funds": "",
            "trades_count": "",
            // "time_in_force": "",
        });

        let expected_structure = expected_structure
            .as_object()
            .unwrap()
            .iter()
            .map(|(k, v)| (k.as_str(), v.clone()))
            .collect::<HashMap<&str, Value>>();

        let (missing_keys, extra_keys) = compare_keys(&json, &expected_structure, "");

        if !missing_keys.is_empty() {
            println!(
                "[test_order_cancel_by_uuid] Missing keys: {missing_keys:?}",
            );
            assert!(false);
        } else {
            println!("[test_order_cancel_by_uuid] No keys are missing");
        }

        if !extra_keys.is_empty() {
            println!("[test_order_cancel_by_uuid] Extra keys: {extra_keys:?}");
            assert!(false);
        } else {
            println!("[test_order_cancel_by_uuid] No extra keys found.");
        }

        assert!(true);
    }

    async fn order_to_get_uuid() -> String {
        crate::set_access_key(&std::env::var("TEST_ACCESS_KEY").expect("TEST_ACCESS_KEY not set"));
        crate::set_secret_key(&std::env::var("TEST_SECRET_KEY").expect("TEST_SECRET_KEY not set"));

        let price = 5000.0;
        let price_desired = 1_435_085.0;

        let res = super::super::order_by_price(
            "KRW-ETH",
            OrderSide::Bid,
            price,
            price_desired,
            OrderType::Limit,
            None,
        )
        .await;

        if let Ok(res) = res {
            res.uuid
        } else {
            panic!("Failed to get uuid from order_by_price")
        }
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
                    let new_path = format!("{path}{key}.");
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
