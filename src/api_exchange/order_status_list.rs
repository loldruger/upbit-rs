use reqwest::{
    header::{ACCEPT, AUTHORIZATION},
    Response, 
    Url
};

use crate::request::RequestWithQuery;
use crate::{constant::OrderBy, request::Request};

use super::{
    super::{constant::{
        URL_ORDER_STATUS_BY_UUID, URL_ORDER_STATUS_CLOSED, URL_ORDER_STATUS_LIST, URL_ORDER_STATUS_OPEN, URL_SERVER
    }, response::{
        OrderInfo,
        OrderInfoSource,
        ResponseError
    }},
    OrderState
};

impl OrderInfo {
    pub async fn get_order_states_by_uuids(market_id: &str, uuids: Vec<&str>, order_by: OrderBy) -> Result<Vec<Self>, ResponseError> {
        let res = Self::request_orders_by_uuids(market_id, uuids, order_by).await?;
        let res_serialized = res.text().await.map_err(crate::response::response_error_from_reqwest)?;
        
        if res_serialized.contains("error") {
            return Err(serde_json::from_str(&res_serialized)
                .map(crate::response::response_error)                
                .ok()
                .unwrap()
            )
        } 

        Self::deserialize_response(res_serialized)
    }

    pub async fn get_order_states_by_identifiers(market_id: &str, identifiers: Vec<&str>, order_by: OrderBy) -> Result<Vec<Self>, ResponseError> {
        let res = Self::request_orders_by_identifiers(market_id, identifiers, order_by).await?;
        let res_serialized = res.text().await.map_err(crate::response::response_error_from_reqwest)?;
        
        if res_serialized.contains("error") {
            return Err(serde_json::from_str(&res_serialized)
                .map(crate::response::response_error)                
                .ok()
                .unwrap()
            )
        } 

        Self::deserialize_response(res_serialized)
    }
    
    pub async fn get_order_states_opened(market_id: &str, state: OrderState, states: Vec<OrderState>, page: u8, limit: u8, order_by: OrderBy) -> Result<Vec<Self>, ResponseError> {
        let res = Self::request_orders_opened(market_id, state, states, page, limit, order_by).await?;
        let res_serialized = res.text().await.map_err(crate::response::response_error_from_reqwest)?;
        
        if res_serialized.contains("error") {
            return Err(serde_json::from_str(&res_serialized)
                .map(crate::response::response_error)                
                .ok()
                .unwrap()
            )
        }

        Self::deserialize_response(res_serialized)
    }

    pub async fn get_order_states_closed(market_id: &str, state: OrderState, start_time: &str, end_time: &str, limit: u16, order_by: OrderBy) -> Result<Vec<Self>, ResponseError> {
        let res = Self::request_orders_closed(market_id, state, start_time, end_time, limit, order_by).await?;
        let res_serialized = res.text().await.map_err(crate::response::response_error_from_reqwest)?;

        if res_serialized.contains("error") {
            return Err(serde_json::from_str(&res_serialized)
                .map(crate::response::response_error)                
                .ok()
                .unwrap()
            )
        }

        Self::deserialize_response(res_serialized)
    }

    #[deprecated(since = "1.6.0")]
    pub async fn get_order_state_list() -> Result<Vec<Self>, ResponseError> {
        let res = Self::request(&format!("{URL_SERVER}{URL_ORDER_STATUS_LIST}")).await?;
        let res_serialized = res.text().await.map_err(crate::response::response_error_from_reqwest)?;
        
        if res_serialized.contains("error") {
            return Err(serde_json::from_str(&res_serialized)
                .map(crate::response::response_error)                
                .ok()
                .unwrap()
            )
        }

        Self::deserialize_response(res_serialized)
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

    async fn request_orders_by_uuids(market_id: &str, uuids: Vec<&str>, order_by: OrderBy) -> Result<Response, ResponseError> {
        let mut url = Url::parse(&format!("{URL_SERVER}{URL_ORDER_STATUS_BY_UUID}")).unwrap();
        
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

    async fn request_orders_by_identifiers(market_id: &str, identifiers: Vec<&str>, order_by: OrderBy) -> Result<Response, ResponseError> {
        let mut url = Url::parse(&format!("{URL_SERVER}{URL_ORDER_STATUS_BY_UUID}")).unwrap();
        
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

    async fn request_orders_opened(market_id: &str, state: OrderState, states: Vec<OrderState>, page: u8, limit: u8, order_by: OrderBy) -> Result<Response, ResponseError> {
        let mut url = Url::parse(&format!("{URL_SERVER}{URL_ORDER_STATUS_OPEN}")).unwrap();
        
        url.query_pairs_mut()
            .append_pair("market", market_id)
            .append_pair("state", &state.to_string())
            .append_pair("states", &states.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","))
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

    async fn request_orders_closed(market_id: &str, state: OrderState, start_time: &str, end_time: &str, limit: u16, order_by: OrderBy) -> Result<Response, ResponseError> {
        let mut url = Url::parse(&format!("{URL_SERVER}{URL_ORDER_STATUS_CLOSED}")).unwrap();
        
        url.query_pairs_mut()
            .append_pair("market", market_id)
            .append_pair("state", &state.to_string())
            .append_pair("start_time", start_time)
            .append_pair("end_time", end_time)
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

    fn deserialize_response(res: String) -> Result<Vec<Self>, ResponseError> {
        serde_json::from_str(&res)
            .map(|i: Vec<OrderInfoSource>| {
                i
                    .into_iter()
                    .map(|x| 
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
                        })
                    .collect::<Vec<Self>>()
            })
            .map_err(crate::response::response_error_from_json)
    }
}

// #[cfg(test)]
// mod tests {
//     use std::collections::HashSet;

//     use super::*;
//     use crate::response::ResponseError;

//     #[tokio::test]
//     async fn test_get_order_states_by_uuids() {
//         crate::set_access_key(&std::env::var("TEST_ACCESS_KEY").expect("TEST_ACCESS_KEY not set"));
//         crate::set_secret_key(&std::env::var("TEST_SECRET_KEY").expect("TEST_SECRET_KEY not set"));
    
//         let market_id = "KRW-BTC";
//         let uuids = vec!["uuid1", "uuid2"];
//         let identifiers = vec!["identifier1", "identifier2"];
//         let order_by = OrderBy::Asc;

//         let res = OrderInfo::request_orders_by_uuids(market_id, &uuids, order_by).await?;
//         let res_serialized = res.text().await.map_err(crate::response::response_error_from_reqwest)?;
        
//     }

//     #[tokio::test]
//     async fn test_get_order_states_opened() {
//         let market_id = "KRW-BTC";
//         let state = OrderState::Wait;
//         let states = vec![OrderState::Wait, OrderState::Done];
//         let page = 1;
//         let limit = 10;
//         let order_by = OrderBy::Asc;

//         let res = OrderInfo::get_order_states_opened(market_id, state, states, page, limit, order_by).await;
//         assert!(res.is_ok());
//     }

//     #[tokio::test]
//     async fn test_get_order_states_closed() {
//         let market_id = "KRW-BTC";
//         let state = OrderState::Wait;
//         let start_time = "2021-01-01T00:00:00Z";
//         let end_time = "2021-01-02T00:00:00Z";
//         let limit = 10;
//         let order_by = OrderBy::Asc;

//         let res = OrderInfo::get_order_states_closed(market_id, state, start_time, end_time, limit, order_by).await;
//         assert!(res.is_ok());
//     }

//     #[tokio::test]
//     async fn test_get_order_state_list() {
//         let res = OrderInfo::get_order_state_list().await;
//         assert!(res.is_ok());
//     }

//     #[tokio::test]
//     async fn test_request() {
//         let url = "https://api.upbit.com/v1/order/status";
//         let res = OrderInfo::request(url).await;
//         assert!(res.is_ok());
//     }

//     #[tokio::test]
//     async fn test_request_orders_by_uuids() {
//         let market_id = "KRW-BTC";
