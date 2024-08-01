use reqwest::{
    Response,
    Url,
    header::{ACCEPT, AUTHORIZATION}
};
use crate::request::RequestWithQuery;

use super::{
    super::constant::{URL_ORDER_STATUS, URL_SERVER},
    super::response::{
        OrderInfo,
        OrderInfoSource,
        ResponseError
    }
};

impl OrderInfo {
    pub async fn cancel_order(uuid: Option<&str>, identifier: Option<&str>) -> Result<Self, ResponseError> {
        if uuid.is_none() && identifier.is_none() {
            return Err(crate::response::response_error_internal_neither_parameter_specified());

        } else if uuid.is_some() && identifier.is_some() {
            return Err(crate::response::response_error_internal_too_many_parameter_specified());
        }

        let res = Self::request_cancel(uuid, identifier).await?;
        let res_serialized = res.text().await.map_err(crate::response::response_error_from_reqwest)?;

        if res_serialized.contains("error") {
            return Err(serde_json::from_str(&res_serialized)
                .map(crate::response::response_error)
                .ok()
                .unwrap()
            )
        }

        serde_json::from_str(&res_serialized)
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

    async fn request_cancel(uuid: Option<&str>, identifier: Option<&str>) -> Result<Response, ResponseError> {
        let mut url = Url::parse(&format!("{URL_SERVER}{URL_ORDER_STATUS}")).unwrap();

        if let Some(uuid) = uuid {
            url.query_pairs_mut().append_pair("uuid", uuid);
        }

        if let Some(identifier) = identifier {
            url.query_pairs_mut().append_pair("identifier", identifier);
        }

        let token_string = Self::set_token_with_query(url.as_str())?;

        reqwest::Client::new()
            .delete(url.as_str())
            .header(ACCEPT, "application/json")
            .header(AUTHORIZATION, &token_string)
            .send()
            .await
            .map_err(crate::response::response_error_from_reqwest)
    }
}

// #[cfg(test)]
// mod tests {
//     use serde_json::Value;

//     use super::*;

//     #[tokio::test]
//     async fn cancel_order() {
//         crate::set_access_key(&std::env::var("TEST_ACCESS_KEY").expect("TEST_ACCESS_KEY not set"));
//         crate::set_secret_key(&std::env::var("TEST_SECRET_KEY").expect("TEST_SECRET_KEY not set"));

//         let res = OrderInfo::request_cancel().await.unwrap();
//         let res_serialized = res
//             .text()
//             .await
//             .map_err(crate::response::response_error_from_reqwest)
//             .unwrap();

//         let json = serde_json::from_str::<Value>(&res_serialized).unwrap();
//         println!("{:#?}", json.get(0));

//         assert_ne!(json.get("uuid"), None);
//         assert_ne!(json.get("side"), None);
//         assert_ne!(json.get("ord_type"), None);
//         assert_ne!(json.get("price"), None);
//         assert_ne!(json.get("state"), None);
//         assert_ne!(json.get("market"), None);
//         assert_ne!(json.get("created_at"), None);
//         assert_ne!(json.get("volume"), None);
//         assert_ne!(json.get("remaining_volume"), None);
//         assert_ne!(json.get("reserved_fee"), None);
//         assert_ne!(json.get("remaining_fee"), None);
//         assert_ne!(json.get("paid_fee"), None);
//         assert_ne!(json.get("locked"), None);
//         assert_ne!(json.get("executed_volume"), None);
//         assert_ne!(json.get("executed_funds"), None);
//         assert_ne!(json.get("trades_count"), None);
//         assert_ne!(json.get("time_in_force"), None);
//     }
// }