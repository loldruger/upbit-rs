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
        let res_serialized = match res.text().await {
            Ok(s) => s,
            Err(e) => return Err(crate::response::response_error_from_reqwest(e))
        };
        
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
                    trades_count: x.trades_count()
                }
            })
            .map_err(crate::response::response_error_from_json)
    }

    async fn request_cancel(uuid: Option<&str>, identifier: Option<&str>) -> Result<Response, ResponseError> {
        let mut url = Url::parse(&format!("{URL_SERVER}{URL_ORDER_STATUS}")).unwrap();

        if uuid.is_some() {
            url.query_pairs_mut().append_pair("uuid", uuid.unwrap());
        }

        if identifier.is_some() {
            url.query_pairs_mut().append_pair("identifier", identifier.unwrap());
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

