use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Url, Response};

use crate::request::RequestWithQuery;

use super::{
    OrderType,
    OrderSide,
    super::{
        constant::{URL_ORDER, URL_SERVER},
        response::{
            OrderInfo,
            OrderInfoSource,
            ResponseError
        }
    }
};

impl OrderInfo {
    pub async fn order_by_price(market_id: &str, side: OrderSide, volume: f64, price: f64, ord_type: OrderType, identifier: Option<&str>) -> Result<Self, ResponseError> {
        let res = Self::request_order_by_price(market_id, side, volume, price, ord_type, identifier).await?;
        let res_serialized = res.text().await.map_err(crate::response::response_error_from_reqwest)?;

        if res_serialized.contains("error") {
            return Err(serde_json::from_str(&res_serialized)
                .map(crate::response::response_error)                
                .ok()
                .unwrap()
            )
        }

        Self::deserialize_order_response(res_serialized)
    }

    pub async fn order_ask_at_market_price(market_id: &str, side: OrderSide, volume: f64, ord_type: OrderType, identifier: Option<&str>) -> Result<Self, ResponseError> {
        let res = Self::request_ask_at_market_price(market_id, side, volume, ord_type, identifier).await?;
        let res_serialized = res.text().await.map_err(crate::response::response_error_from_reqwest)?;

        if res_serialized.contains("error") {
            return Err(serde_json::from_str(&res_serialized)
                .map(crate::response::response_error)                
                .ok()
                .unwrap()
            )
        }

        Self::deserialize_order_response(res_serialized)
    }

    async fn request_order_by_price(market_id: &str, side: OrderSide, volume: f64, price: f64, ord_type: OrderType, identifier: Option<&str>) -> Result<Response, ResponseError> {
        let mut url = Url::parse(&format!("{URL_SERVER}{URL_ORDER}")).unwrap();
        let price = format!("{:.8}", price);
        let volume = format!("{:.8}", volume);

        url.query_pairs_mut()
            .append_pair("market", market_id)
            .append_pair("side", &side.to_string())
            .append_pair("ord_type", &ord_type.to_string())
            .append_pair("price", &price.to_string())
            .append_pair("volume", &volume.to_string());

        if let Some(identifier) = identifier {
            url.query_pairs_mut().append_pair("identifier", identifier);
        }

        let token_string = Self::set_token_with_query(url.as_str())?;
        
        reqwest::Client::new()
            .post(url.as_str())
            .header(ACCEPT, "application/json")
            .header(CONTENT_TYPE, "application/json")
            .header(AUTHORIZATION, &token_string)
            .send()
            .await
            .map_err(crate::response::response_error_from_reqwest)
    }

    async fn request_ask_at_market_price(market_id: &str, side: OrderSide, volume: f64, ord_type: OrderType, identifier: Option<&str>) -> Result<Response, ResponseError> {
        let mut url = Url::parse(&format!("{URL_SERVER}{URL_ORDER}")).unwrap();
        let volume = format!("{:.8}", volume);

        url.query_pairs_mut()
            .append_pair("market", market_id)
            .append_pair("side", &side.to_string())
            .append_pair("ord_type", &ord_type.to_string())
            .append_pair("volume", &volume.to_string());

        if let Some(identifier) = identifier {
            url.query_pairs_mut().append_pair("identifier", identifier);
        }

        let token_string = Self::set_token_with_query(url.as_str())?;
        
        reqwest::Client::new()
            .post(url.as_str())
            .header(ACCEPT, "application/json")
            .header(CONTENT_TYPE, "application/json")
            .header(AUTHORIZATION, &token_string)
            .send()
            .await
            .map_err(crate::response::response_error_from_reqwest)
    }

    async fn request_bid_at_market_price(market_id: &str, side: OrderSide, price: f64, ord_type: OrderType, identifier: Option<&str>) -> Result<Response, ResponseError> {
        let mut url = Url::parse(&format!("{URL_SERVER}{URL_ORDER}")).unwrap();
        let price = format!("{:.8}", price);

        url.query_pairs_mut()
            .append_pair("market", market_id)
            .append_pair("side", &side.to_string())
            .append_pair("ord_type", &ord_type.to_string())
            .append_pair("price", &price.to_string());

        if let Some(identifier) = identifier {
            url.query_pairs_mut().append_pair("identifier", identifier);
        }

        let token_string = Self::set_token_with_query(url.as_str())?;
        
        reqwest::Client::new()
            .post(url.as_str())
            .header(ACCEPT, "application/json")
            .header(CONTENT_TYPE, "application/json")
            .header(AUTHORIZATION, &token_string)
            .send()
            .await
            .map_err(crate::response::response_error_from_reqwest)
    }

    fn deserialize_order_response(res: String) -> Result<Self, ResponseError> {
        serde_json::from_str(&res)
        .map(|x: OrderInfoSource| {
            Self {
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
            }
        })
        .map_err(crate::response::response_error_from_json)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use serde_json::Value;

    use crate::api_exchange::price_checker;

    use super::*;

    #[tokio::test]
    async fn test_order_bid_by_price() {
        crate::set_access_key(&std::env::var("TEST_ACCESS_KEY").expect("TEST_ACCESS_KEY not set"));
        crate::set_secret_key(&std::env::var("TEST_SECRET_KEY").expect("TEST_SECRET_KEY not set"));

        let volume = (5000.0 + 1.) / price_checker(1_435_085.0);
        let price = price_checker(1_435_085.0);

        let res = OrderInfo::request_order_by_price("KRW-ETH", OrderSide::Bid, volume, price, OrderType::Limit, None).await.unwrap();
        let res_serialized = res.text().await.map_err(crate::response::response_error_from_reqwest).unwrap();
        
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
            println!("[test_order_bid_by_price] Missing keys: {:?}", missing_keys);
            assert!(false);
        } else {
            println!("[test_order_bid_by_price] No keys are missing");
            assert!(true);
        }
    
        if !extra_keys.is_empty() {
            println!("[test_order_bid_by_price] Extra keys: {:?}", extra_keys);
            assert!(false);
        } else {
            println!("[test_order_bid_by_price] No extra keys found.");
            assert!(true);
        }
    }

    #[tokio::test]
    async fn test_order_ask_by_price() {
        crate::set_access_key(&std::env::var("TEST_ACCESS_KEY").expect("TEST_ACCESS_KEY not set"));
        crate::set_secret_key(&std::env::var("TEST_SECRET_KEY").expect("TEST_SECRET_KEY not set"));

        let volume = (5000.0 + 1.) / price_checker(3_435_085.0);
        let price = price_checker(3_435_085.0);

        let res = OrderInfo::request_order_by_price("KRW-ETH", OrderSide::Ask, volume, price, OrderType::Limit, None).await.unwrap();
        let res_serialized = res.text().await.map_err(crate::response::response_error_from_reqwest).unwrap();
        
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
            println!("[test_order_ask_by_price] Missing keys: {:?}", missing_keys);
            assert!(false);
        } else {
            println!("[test_order_ask_by_price] No keys are missing");
            assert!(true);
        }
    
        if !extra_keys.is_empty() {
            println!("[test_order_ask_by_price] Extra keys: {:?}", extra_keys);
            assert!(false);
        } else {
            println!("[test_order_ask_by_price] No extra keys found.");
            assert!(true);
        }
    }

    #[tokio::test]
    async fn test_order_bid_at_market_price() {
        crate::set_access_key(&std::env::var("TEST_ACCESS_KEY").expect("TEST_ACCESS_KEY not set"));
        crate::set_secret_key(&std::env::var("TEST_SECRET_KEY").expect("TEST_SECRET_KEY not set"));

        let price = price_checker(5000.0);

        let res = OrderInfo::request_bid_at_market_price("KRW-ETH", OrderSide::Bid, price, OrderType::Price, None).await.unwrap();
        let res_serialized = res.text().await.map_err(crate::response::response_error_from_reqwest).unwrap();

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
            // "volume": "",
            // "remaining_volume": "",
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
            println!("[test_order_bid_at_market_price] Missing keys: {:?}", missing_keys);
            assert!(false);
        } else {
            println!("[test_order_bid_at_market_price] No keys are missing");
            assert!(true);
        }

        if !extra_keys.is_empty() {
            println!("[test_order_bid_at_market_price] Extra keys: {:?}", extra_keys);
            assert!(false);
        } else {
            println!("[test_order_bid_at_market_price] No extra keys found.");
            assert!(true);
        }
    }

    #[tokio::test]
    async fn test_order_ask_at_market_price() {
        crate::set_access_key(&std::env::var("TEST_ACCESS_KEY").expect("TEST_ACCESS_KEY not set"));
        crate::set_secret_key(&std::env::var("TEST_SECRET_KEY").expect("TEST_SECRET_KEY not set"));

        let volume = 0.0015;

        let res = OrderInfo::request_ask_at_market_price("KRW-ETH", OrderSide::Ask, volume, OrderType::Market, None).await.unwrap();
        let res_serialized = res.text().await.map_err(crate::response::response_error_from_reqwest).unwrap();

        if res_serialized.contains("error") {
            assert!(false, "Error response: {res_serialized}");
        }

        let json = serde_json::from_str::<Value>(&res_serialized).unwrap();
        let expected_structure = serde_json::json!({
            "uuid": "",
            "side": "",
            "ord_type": "",
            // "price": "",
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
            println!("[test_order_ask_at_market_price] Missing keys: {:?}", missing_keys);
            assert!(false);
        } else {
            println!("[test_order_ask_at_market_price] No keys are missing");
            assert!(true);
        }

        if !extra_keys.is_empty() {
            println!("[test_order_ask_at_market_price] Extra keys: {:?}", extra_keys);
            assert!(false);
        } else {
            println!("[test_order_ask_at_market_price] No extra keys found.");
            assert!(true);
        }
    }

    // async fn test order_bid_by_price
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