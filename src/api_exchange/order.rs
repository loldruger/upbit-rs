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
            ResponseError,
            ResponseErrorBody,
            ResponseErrorState
        }
    }
};

impl OrderInfo {
    pub async fn order(market_id: &str, side: OrderSide, volume: Option<f64>, price: Option<f64>, ord_type: OrderType, identifier: Option<&str>) -> Result<Self, ResponseError> {
        let res = Self::request_order(market_id, side, volume, price, ord_type, identifier).await?;
        let res_serialized = res.text().await.unwrap();
        
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
                    trades_count: x.trades_count(),
                }
            })
            .map_err(crate::response::response_error_from_json)
    }

    async fn request_order(market_id: &str, side: OrderSide, volume: Option<f64>, price: Option<f64>, ord_type: OrderType, identifier: Option<&str>) -> Result<Response, ResponseError> {
        let mut url = Url::parse(&format!("{URL_SERVER}{URL_ORDER}")).unwrap();
        
        url.query_pairs_mut()
            .append_pair("market", market_id)
            .append_pair("side", &side.to_string())
            .append_pair("ord_type", &ord_type.to_string());
            
        if price.is_some() {
            let price = format!("{:.8}", price.unwrap());
            url.query_pairs_mut().append_pair("price", price.as_str());
        }
        
        if volume.is_some() {
            let volume = format!("{:.8}", volume.unwrap()); 
            url.query_pairs_mut().append_pair("volume", volume.as_str());
        }

        if identifier.is_some() {
            url.query_pairs_mut().append_pair("identifier", identifier.unwrap());
        }

        // let asdf: Option<String> = if let Some(x) = url.query() {
        //     let mut y = x.replace('=', ":");
        //     y = y.replace('&', ",");
        //     y.insert(0, '{');
        //     y.insert(y.len(), '}');

        //     Some(y)
        // } else {
        //     None
        // };

        let token_string = Self::set_token_with_query(url.as_str())?;
        
        reqwest::Client::new()
            .post(url.as_str())
            // .json(&asdf)
            .header(ACCEPT, "application/json")
            .header(CONTENT_TYPE, "application/json")
            .header(AUTHORIZATION, &token_string)
            .send()
            .await
            .map_err(|x| {
                ResponseError {
                    state: ResponseErrorState::InternalReqwestError,
                    error: ResponseErrorBody {
                        name: "internal_reqwest_error".to_owned(),
                        message: x.to_string()
                    }
                }
            })
    }
}
