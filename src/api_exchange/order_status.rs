use reqwest::header::{ACCEPT, AUTHORIZATION};
use reqwest::{Url, Response};

use crate::request::RequestWithQuery;

use super::{
    super::constant::{URL_ORDER_STATUS, URL_SERVER},
    super::response::{
        OrderInfo,
        OrderStatus,
        OrderStatusSource,
        ObjectTrades,
        ResponseError
    }
};

impl RequestWithQuery for OrderStatus {}
impl OrderStatus {
    pub async fn get_order_status_by_uuid(uuid: &str) -> Result<Self, ResponseError> {
        let res = Self::request_by_uuid(uuid).await?;
        let res_serialized = res.text().await.map_err(crate::response::response_error_from_reqwest)?;
        
        if res_serialized.contains("error") {
            return Err(serde_json::from_str(&res_serialized)
                .map(crate::response::response_error)
                .ok()
                .unwrap()
            )
        }
        
        Self::deserialize_order_status(res_serialized)
    }

    pub async fn get_order_status_by_identifier(identifier: &str) -> Result<Self, ResponseError> {
        let res = Self::request_by_identifier(identifier).await?;
        let res_serialized = res.text().await.map_err(crate::response::response_error_from_reqwest)?;
        
        if res_serialized.contains("error") {
            return Err(serde_json::from_str(&res_serialized)
                .map(crate::response::response_error)
                .ok()
                .unwrap()
            )
        }
        
        Self::deserialize_order_status(res_serialized)
    }
    
    async fn request_by_uuid(uuid: &str, ) -> Result<Response, ResponseError> {
        let mut url = Url::parse(&format!("{URL_SERVER}{URL_ORDER_STATUS}")).unwrap();
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

    async fn request_by_identifier(identifier: &str) -> Result<Response, ResponseError> {
        let mut url = Url::parse(&format!("{URL_SERVER}{URL_ORDER_STATUS}")).unwrap();
        url.query_pairs_mut().append_pair("identifier", identifier);

        let token_string = Self::set_token_with_query(url.as_str())?;

        reqwest::Client::new()
            .get(url.as_str())
            .header(ACCEPT, "application/json")
            .header(AUTHORIZATION, &token_string)
            .send()
            .await
            .map_err(crate::response::response_error_from_reqwest)
    }

    fn deserialize_order_status(res_serialized: String) -> Result<Self, ResponseError> {
        serde_json::from_str(&res_serialized)
            .map(|x: OrderStatusSource| {
                Self {
                    order_info: OrderInfo {
                        uuid: x.order_info.uuid(),
                        side: x.order_info.side(),
                        ord_type: x.order_info.ord_type(),
                        price: x.order_info.price(),
                        state: x.order_info.state(),
                        market: x.order_info.market(),
                        created_at: x.order_info.created_at(),
                        volume: x.order_info.volume(),
                        remaining_volume: x.order_info.remaining_volume(),
                        reserved_fee: x.order_info.reserved_fee(),
                        remaining_fee: x.order_info.remaining_fee(),
                        paid_fee: x.order_info.paid_fee(),
                        locked: x.order_info.locked(),
                        executed_volume: x.order_info.executed_volume(),
                        executed_funds: x.order_info.executed_funds(),
                        trades_count: x.order_info.trades_count(),
                        time_in_force: x.order_info.time_in_force(),
                    },
                    trades: x.trades
                        .into_iter()
                        .map(|object_trades| ObjectTrades {
                            market: object_trades.market,
                            uuid: object_trades.uuid,
                            price: object_trades.price.parse().unwrap_or(0.0),
                            volume: object_trades.volume.parse().unwrap_or(0.0),
                            funds: object_trades.funds.parse().unwrap_or(0.0),
                            side: object_trades.side,
                            created_at: object_trades.created_at,
                        })
                        .collect(),
                }
            })
            .map_err(crate::response::response_error_from_json)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use crate::api_exchange::{price_checker, OrderSide, OrderType};

    use super::*;

    use serde_json::Value;

    #[tokio::test]
    async fn test_get_order_status_by_uuid() {
        crate::set_access_key(&std::env::var("TEST_ACCESS_KEY").expect("TEST_ACCESS_KEY not set"));
        crate::set_secret_key(&std::env::var("TEST_SECRET_KEY").expect("TEST_SECRET_KEY not set"));
        
        let uuid = order_to_get_uuid().await;

        let res = OrderStatus::request_by_uuid(&uuid).await.unwrap();
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
            "trades": [
                {
                    "market": "",
                    "uuid": "",
                    "price": "",
                    "volume": "",
                    "funds": "",
                    "side": "",
                    "created_at": "",
                }
            ]
        });

        let expected_structure = expected_structure
            .as_object()
            .unwrap()
            .iter()
            .map(|(k, v)| (k.as_str(), v.clone()))
            .collect::<HashMap<&str, Value>>();

            let (missing_keys, extra_keys) = compare_keys(&json, &expected_structure, "");
    
            if !missing_keys.is_empty() {
                println!("[test_get_order_status_by_uuid] Missing keys: {:?}", missing_keys);
                assert!(false);
            } else {
                println!("[test_get_order_status_by_uuid] No keys are missing");
                assert!(true);
            }
        
            if !extra_keys.is_empty() {
                println!("[test_get_order_status_by_uuid] Extra keys: {:?}", extra_keys);
                assert!(false);
            } else {
                println!("[test_get_order_status_by_uuid] No extra keys found.");
                assert!(true);
            }
    
    }

    #[tokio::test]
    async fn test_get_order_status_by_identifier() {
        crate::set_access_key(&std::env::var("TEST_ACCESS_KEY").expect("TEST_ACCESS_KEY not set"));
        crate::set_secret_key(&std::env::var("TEST_SECRET_KEY").expect("TEST_SECRET_KEY not set"));
    
        let identifier = order_to_get_identifier().await;

        let res = OrderStatus::request_by_identifier(&identifier).await.unwrap();
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
            "executed_funds": "",
            "trades_count": "",
            "time_in_force": "",
            "trades": [
                {
                    "market": "",
                    "uuid": "",
                    "price": "",
                    "volume": "",
                    "funds": "",
                    "side": "",
                    "created_at": "",
                }
            ]
        });

        let expected_structure = expected_structure
            .as_object()
            .unwrap()
            .iter()
            .map(|(k, v)| (k.as_str(), v.clone()))
            .collect::<HashMap<&str, Value>>();

        let (missing_keys, extra_keys) = compare_keys(&json, &expected_structure, "");

        if !missing_keys.is_empty() {
            println!("[test_get_order_status_by_identifier] Missing keys: {:?}", missing_keys);
            assert!(false);
        } else {
            println!("[test_get_order_status_by_identifier] No keys are missing");
            assert!(true);
        }
    
        if !extra_keys.is_empty() {
            println!("[test_get_order_status_by_identifier] Extra keys: {:?}", extra_keys);
            assert!(false);
        } else {
            println!("[test_get_order_status_by_identifier] No extra keys found.");
            assert!(true);
        }
    }

    async fn order_to_get_uuid() -> String {
        crate::set_access_key(&std::env::var("TEST_ACCESS_KEY").expect("TEST_ACCESS_KEY not set"));
        crate::set_secret_key(&std::env::var("TEST_SECRET_KEY").expect("TEST_SECRET_KEY not set"));
        
        let price = 5000.0;
        let price_desired = 1_435_085.0;

        let res = super::super::order_by_price("KRW-ETH", OrderSide::Bid, price, price_desired, OrderType::Limit, None).await;

        if let Ok(res) = res {
            res.uuid
        } else {
            println!("{:?}", res);
            panic!("Failed to get uuid from order_by_price")
        }
    }

    async fn order_to_get_identifier() -> String {
        crate::set_access_key(&std::env::var("TEST_ACCESS_KEY").expect("TEST_ACCESS_KEY not set"));
        crate::set_secret_key(&std::env::var("TEST_SECRET_KEY").expect("TEST_SECRET_KEY not set"));
        
        let price = 5000.0;
        let price_desired = 1_435_085.0;
        let identifier = "test_identifier2".to_string();

        let res = super::super::order_by_price("KRW-ETH", OrderSide::Bid, price, price_desired, OrderType::Limit, Some(&identifier)).await;

        if let Ok(_) = res {
            identifier
        } else {
            println!("{:?}", res);
            panic!("Failed to tag identifier from order_by_price")
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
