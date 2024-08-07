use reqwest::{
    header::{ACCEPT, AUTHORIZATION},
    Response, Url,
};

use crate::request::RequestWithQuery;
use crate::{constant::OrderBy, request::Request};

#[allow(deprecated)]
use super::{
    super::{
        constant::{
            URL_ORDER_STATUS_BY_UUID, URL_ORDER_STATUS_CLOSED, URL_ORDER_STATUS_LIST,
            URL_ORDER_STATUS_OPEN, URL_SERVER,
        },
        response::{OrderInfo, OrderInfoSource, ResponseError},
    },
    OrderState,
};

impl OrderInfo {
    pub async fn get_order_states_by_uuids(
        market_id: &str,
        uuids: &[&str],
        order_by: OrderBy,
    ) -> Result<Vec<Self>, ResponseError> {
        let res = Self::request_get_orders_by_uuids(market_id, uuids, order_by).await?;
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

        Self::deserialize_order_states_response(res_serialized)
    }

    pub async fn get_order_states_by_identifiers(
        market_id: &str,
        identifiers: &[&str],
        order_by: OrderBy,
    ) -> Result<Vec<Self>, ResponseError> {
        let res = Self::request_get_orders_by_identifiers(market_id, identifiers, order_by).await?;
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

        Self::deserialize_order_states_response(res_serialized)
    }

    pub async fn get_order_states_opened(
        market_id: &str,
        state: OrderState,
        states: &[OrderState],
        page: u8,
        limit: u8,
        order_by: OrderBy,
    ) -> Result<Vec<Self>, ResponseError> {
        let res = Self::request_get_orders_opened(market_id, state, states, page, limit, order_by)
            .await?;
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

        Self::deserialize_order_states_response(res_serialized)
    }

    pub async fn get_order_states_closed(
        market_id: &str,
        state: OrderState,
        start_time: Option<&str>,
        end_time: Option<&str>,
        limit: u16,
        order_by: OrderBy,
    ) -> Result<Vec<Self>, ResponseError> {
        let res = Self::request_get_orders_closed(
            market_id, state, start_time, end_time, limit, order_by,
        )
        .await?;
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

        Self::deserialize_order_states_response(res_serialized)
    }

    #[deprecated(since = "1.6.0")]
    pub async fn get_order_state_list() -> Result<Vec<Self>, ResponseError> {
        #[allow(deprecated)]
        let res = Self::request(&format!("{URL_SERVER}{URL_ORDER_STATUS_LIST}")).await?;
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

        Self::deserialize_order_states_response(res_serialized)
    }

    #[deprecated(since = "1.6.0")]
    async fn request(url: &str) -> Result<Response, ResponseError> {
        let url = Url::parse(url).unwrap();
        let token_string = Self::set_token()?;

        reqwest::Client::new()
            .get(url.as_str())
            .header(ACCEPT, "application/json")
            .header(AUTHORIZATION, &token_string)
            .send()
            .await
            .map_err(crate::response::response_error_from_reqwest)
    }

    async fn request_get_orders_by_uuids(
        market_id: &str,
        uuids: &[&str],
        order_by: OrderBy,
    ) -> Result<Response, ResponseError> {
        let mut url = Url::parse(&format!("{URL_SERVER}{URL_ORDER_STATUS_BY_UUID}"))
            .map_err(crate::response::response_error_internal_url_parse_error)?;

        url.query_pairs_mut()
            .append_pair("market", market_id)
            .append_pair("order_by", &order_by.to_string());

        for uuid in uuids {
            url.query_pairs_mut().append_pair("uuids", uuid);
        }

        let url = url.as_str().replace("uuids", "uuids[]");
        let token_string = Self::set_token_with_query(&url)?;

        reqwest::Client::new()
            .get(url.as_str())
            .header(ACCEPT, "application/json")
            .header(AUTHORIZATION, &token_string)
            .send()
            .await
            .map_err(crate::response::response_error_from_reqwest)
    }

    async fn request_get_orders_by_identifiers(
        market_id: &str,
        identifiers: &[&str],
        order_by: OrderBy,
    ) -> Result<Response, ResponseError> {
        let mut url = Url::parse(&format!("{URL_SERVER}{URL_ORDER_STATUS_BY_UUID}"))
            .map_err(crate::response::response_error_internal_url_parse_error)?;

        url.query_pairs_mut()
            .append_pair("market", market_id)
            .append_pair("order_by", &order_by.to_string());

        for identifier in identifiers {
            url.query_pairs_mut().append_pair("identifiers", identifier);
        }

        let url = url.as_str().replace("identifiers", "identifiers[]");
        let token_string = Self::set_token_with_query(&url)?;

        reqwest::Client::new()
            .get(url.as_str())
            .header(ACCEPT, "application/json")
            .header(AUTHORIZATION, &token_string)
            .send()
            .await
            .map_err(crate::response::response_error_from_reqwest)
    }

    async fn request_get_orders_opened(
        market_id: &str,
        state: OrderState,
        states: &[OrderState],
        page: u8,
        limit: u8,
        order_by: OrderBy,
    ) -> Result<Response, ResponseError> {
        let mut url = Url::parse(&format!("{URL_SERVER}{URL_ORDER_STATUS_OPEN}"))
            .map_err(crate::response::response_error_internal_url_parse_error)?;

        url.query_pairs_mut()
            .append_pair("market", market_id)
            .append_pair("state", &state.to_string())
            .append_pair(
                "states",
                &states
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            )
            .append_pair("page", &page.to_string())
            .append_pair("limit", &limit.to_string())
            .append_pair("order_by", &order_by.to_string());

        let token_string = Self::set_token_with_query(url.as_str())?;

        reqwest::Client::new()
            .get(url.as_str())
            .header(ACCEPT, "application/json")
            .header(AUTHORIZATION, &token_string)
            .send()
            .await
            .map_err(crate::response::response_error_from_reqwest)
    }

    async fn request_get_orders_closed(
        market_id: &str,
        state: OrderState,
        start_time: Option<&str>,
        end_time: Option<&str>,
        limit: u16,
        order_by: OrderBy,
    ) -> Result<Response, ResponseError> {
        let mut url = Url::parse(&format!("{URL_SERVER}{URL_ORDER_STATUS_CLOSED}"))
            .map_err(crate::response::response_error_internal_url_parse_error)?;

        url.query_pairs_mut()
            .append_pair("market", market_id)
            .append_pair("state", &state.to_string())
            .append_pair("limit", &limit.to_string())
            .append_pair("order_by", &order_by.to_string());

        if let Some(start_time) = start_time {
            url.query_pairs_mut().append_pair("start_time", start_time);
        }

        if let Some(end_time) = end_time {
            url.query_pairs_mut().append_pair("end_time", end_time);
        }

        let token_string = Self::set_token_with_query(url.as_str())?;

        reqwest::Client::new()
            .get(url.as_str())
            .header(ACCEPT, "application/json")
            .header(AUTHORIZATION, &token_string)
            .send()
            .await
            .map_err(crate::response::response_error_from_reqwest)
    }

    fn deserialize_order_states_response(res: String) -> Result<Vec<Self>, ResponseError> {
        serde_json::from_str(&res)
            .map(|i: Vec<OrderInfoSource>| {
                i.into_iter()
                    .map(|x| Self {
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
                    .collect::<Vec<Self>>()
            })
            .map_err(crate::response::response_error_from_json)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json::{json, Value};

    use crate::api_exchange::{OrderSide, OrderState, OrderType};
    #[allow(deprecated)]
    use crate::constant::{OrderBy, URL_ORDER_STATUS_LIST, URL_SERVER};
    use crate::response::OrderInfo;

    #[tokio::test]
    async fn test_get_order_state_list() {
        crate::set_access_key(&std::env::var("TEST_ACCESS_KEY").expect("TEST_ACCESS_KEY not set"));
        crate::set_secret_key(&std::env::var("TEST_SECRET_KEY").expect("TEST_SECRET_KEY not set"));

        #[allow(deprecated)]
        let res = OrderInfo::request(&format!("{URL_SERVER}{URL_ORDER_STATUS_LIST}"))
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
                        "[test_get_order_state_list] Missing keys in item[{}]: {:?}",
                        index, missing_keys
                    );
                    assert!(false);
                } else {
                    println!(
                        "[test_get_order_state_list] No keys are missing in item[{}]",
                        index
                    );
                }

                if !extra_keys.is_empty() {
                    println!(
                        "[test_get_order_state_list] Extra keys in item[{}]: {:?}",
                        index, extra_keys
                    );
                    assert!(false);
                } else {
                    println!(
                        "[test_get_order_state_list] No extra keys found in item[{}]",
                        index
                    );
                }
            }
        } else {
            assert!(false, "Expected an array of objects in the response");
        }

        assert!(true);
    }

    #[tokio::test]
    async fn test_get_orders_by_uuids() {
        crate::set_access_key(&std::env::var("TEST_ACCESS_KEY").expect("TEST_ACCESS_KEY not set"));
        crate::set_secret_key(&std::env::var("TEST_SECRET_KEY").expect("TEST_SECRET_KEY not set"));

        let uuid = order_to_get_uuid().await;

        let res =
            OrderInfo::request_get_orders_by_uuids("KRW-ETH", &[uuid.as_str()], OrderBy::Desc)
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
                        "[test_get_orders_by_uuids] Missing keys in item[{}]: {:?}",
                        index, missing_keys
                    );
                    assert!(false);
                } else {
                    println!(
                        "[test_get_orders_by_uuids] No keys are missing in item[{}]",
                        index
                    );
                }

                if !extra_keys.is_empty() {
                    println!(
                        "[test_get_orders_by_uuids] Extra keys in item[{}]: {:?}",
                        index, extra_keys
                    );
                    assert!(false);
                } else {
                    println!(
                        "[test_get_orders_by_uuids] No extra keys found in item[{}]",
                        index
                    );
                }
            }
        } else {
            assert!(false, "Expected an array of objects in the response");
        }

        assert!(true);
    }

    #[tokio::test]
    async fn test_get_order_states_opened() {
        crate::set_access_key(&std::env::var("TEST_ACCESS_KEY").expect("TEST_ACCESS_KEY not set"));
        crate::set_secret_key(&std::env::var("TEST_SECRET_KEY").expect("TEST_SECRET_KEY not set"));

        let res = OrderInfo::request_get_orders_opened(
            "KRW-ETH",
            OrderState::Wait,
            &[OrderState::Wait],
            1,
            10,
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
            // "time_in_force": "",
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
                        "[test_get_order_states_opened] Missing keys in item[{}]: {:?}",
                        index, missing_keys
                    );
                    assert!(false);
                } else {
                    println!(
                        "[test_get_order_states_opened] No keys are missing in item[{}]",
                        index
                    );
                }

                if !extra_keys.is_empty() {
                    println!(
                        "[test_get_order_states_opened] Extra keys in item[{}]: {:?}",
                        index, extra_keys
                    );
                    assert!(false);
                } else {
                    println!(
                        "[test_get_order_states_opened] No extra keys found in item[{}]",
                        index
                    );
                }
            }
        } else {
            assert!(false, "Expected an array of objects in the response");
        }

        assert!(true);
    }

    #[tokio::test]
    async fn test_get_order_states_closed() {
        crate::set_access_key(&std::env::var("TEST_ACCESS_KEY").expect("TEST_ACCESS_KEY not set"));
        crate::set_secret_key(&std::env::var("TEST_SECRET_KEY").expect("TEST_SECRET_KEY not set"));

        let res = OrderInfo::request_get_orders_closed(
            "KRW-ETH",
            OrderState::Done,
            None,
            None,
            10,
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
            // "time_in_force": "",
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
                    compare_keys(item, &expected_structure, &format!("item[{}]", index));

                let ord_type = item.get("ord_type").and_then(|v| v.as_str()).unwrap();

                if !missing_keys.is_empty() {
                    match ord_type {
                        "limit" => {
                            let missing_keys = missing_keys
                                .iter()
                                .filter(|x| x.contains("price"))
                                .map(|x| x.to_string())
                                .collect::<Vec<String>>();

                            if missing_keys.is_empty() {
                                continue;
                            }
                        }
                        "market" => {
                            continue;
                        }
                        _ => {}
                    }

                    // Check the presence of the price field based on the ord_type field
                    println!(
                        "[test_get_order_states_closed] Missing keys in item[{}]: {:?}",
                        index, missing_keys
                    );
                    assert!(false, "Missing keys found");
                }

                if !extra_keys.is_empty() {
                    println!(
                        "[test_get_order_states_closed] Extra keys in item[{}]: {:?}",
                        index, extra_keys
                    );
                    assert!(false, "Extra keys found");
                }
            }
        } else {
            assert!(false, "Expected an array of objects in the response");
        }
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
