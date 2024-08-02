use reqwest::Response;
use reqwest::header::{ACCEPT, AUTHORIZATION};

use crate::request::RequestWithQuery;
use super::{
    super::constant::{URL_ORDER_CHANCE, URL_SERVER},
    super::response::{
        AccountsInfo,
        OrderChance,
        OrderChanceSource,
        ObjectMarket,
        ObjectAskBid,
        ResponseError
    },
};

impl RequestWithQuery for OrderChance {}
impl OrderChance {
    pub async fn get_order_chance(market_id: &str) -> Result<Self, ResponseError> {
        let res = Self::request(market_id).await?;
        let res_serialized = res.text().await.map_err(crate::response::response_error_from_reqwest)?;

        if res_serialized.contains("error") {
            return Err(serde_json::from_str(&res_serialized)
                .map(crate::response::response_error)
                .ok()
                .unwrap()
            )
        }

        serde_json::from_str(&res_serialized)
            .map(|x: OrderChanceSource| {
                Self {
                    bid_fee: x.bid_fee.parse().unwrap(),
                    ask_fee: x.ask_fee.parse().unwrap(),
                    market: ObjectMarket {
                        id: x.market.id.to_owned(),
                        name: x.market.name.to_owned(),
                        // order_types: x.market.order_types.into_iter().map(|x|x.as_str().into()).collect(),
                        order_sides: x.market.order_sides.into_iter().map(|x|x.as_str().into()).collect(),
                        bid: ObjectAskBid {
                            currency: x.market.bid.currency.to_owned(),
                            price_unit: x.market.bid.price_unit.to_owned(),
                            min_total: x.market.bid.min_total.parse().unwrap(),
                        },
                        ask: ObjectAskBid {
                            currency: x.market.ask.currency.to_owned(),
                            price_unit: x.market.ask.price_unit.to_owned(),
                            min_total: x.market.ask.min_total.parse().unwrap(),
                        },
                        max_total: x.market.max_total.parse().unwrap(),
                        state: x.market.state.to_owned(),
                    },
                    ask_types: None,
                    bid_types: None,
                    bid_account: AccountsInfo {
                        currency: x.bid_account.currency(),
                        balance: x.bid_account.balance(),
                        locked: x.bid_account.locked(),
                        avg_buy_price: x.bid_account.avg_buy_price(),
                        avg_buy_price_modified: x.bid_account.avg_buy_price_modified(),
                        unit_currency: x.bid_account.unit_currency(),
                    },
                    ask_account: AccountsInfo {
                        currency: x.ask_account.currency(),
                        balance: x.ask_account.balance(),
                        locked: x.ask_account.locked(),
                        avg_buy_price: x.ask_account.avg_buy_price(),
                        avg_buy_price_modified: x.ask_account.avg_buy_price_modified(),
                        unit_currency: x.ask_account.unit_currency(),
                    },
                }
            })
            .map_err(crate::response::response_error_from_json)
    }

    async fn request(market_id: &str) -> Result<Response, ResponseError> {
        let url = format!("{URL_SERVER}{URL_ORDER_CHANCE}/?market={market_id}");
        let token_string = Self::set_token_with_query(&url)?;

        reqwest::Client::new()
            .get(url)
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

    use super::*;

    #[tokio::test]
    async fn test_get_order_chance() {
        crate::set_access_key(&std::env::var("TEST_ACCESS_KEY").expect("TEST_ACCESS_KEY not set"));
        crate::set_secret_key(&std::env::var("TEST_SECRET_KEY").expect("TEST_SECRET_KEY not set"));
    
        let res = OrderChance::request("KRW-ETH").await.unwrap();
        let res_serialized = res
            .text()
            .await
            .map_err(crate::response::response_error_from_reqwest)
            .unwrap();
    
        let json = serde_json::from_str::<Value>(&res_serialized).unwrap();
        let expected_structure = serde_json::json!({
            "bid_fee": "",
            "ask_fee": "",
            "market": {
                "id": "",
                "name": "",
                "order_sides": [],
                "order_types": [],
                "bid": {
                    "currency": "",
                    "min_total": ""
                },
                "ask": {
                    "currency": "",
                    "min_total": ""
                },
                "max_total": "",
                "state": "",
                "ask_types": [],
                "bid_types": [],
            },
            "maker_ask_fee": "",
            "maker_bid_fee": "",
            "bid_account": {
                "currency": "",
                "balance": "",
                "locked": "",
                "avg_buy_price": "",
                "avg_buy_price_modified": "",
                "unit_currency": ""
            },
            "ask_account": {
                "currency": "",
                "balance": "",
                "locked": "",
                "avg_buy_price": "",
                "avg_buy_price_modified": "",
                "unit_currency": ""
            }
        });
    
        let expected_structure = expected_structure
            .as_object()
            .unwrap()
            .iter()
            .map(|(k, v)| (k.as_str(), v.clone()))
            .collect::<HashMap<&str, Value>>();
    
        let (missing_keys, extra_keys) = compare_keys(&json, &expected_structure, "");
    
        if !missing_keys.is_empty() {
            println!("[test_get_order_chance] Missing keys: {:?}", missing_keys);
            assert!(false);
        } else {
            println!("[test_get_order_chance] No keys are missing");
            assert!(true);
        }
    
        if !extra_keys.is_empty() {
            println!("[test_get_order_chance] Extra keys: {:?}", extra_keys);
            assert!(false);
        } else {
            println!("[test_get_order_chance] No extra keys found.");
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