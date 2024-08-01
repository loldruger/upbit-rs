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
    pub async fn order(market_id: &str, side: OrderSide, volume: Option<f64>, price: Option<f64>, ord_type: OrderType, identifier: Option<&str>) -> Result<Self, ResponseError> {
        let res = Self::request_order(market_id, side, volume, price, ord_type, identifier).await?;
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

    async fn request_order(market_id: &str, side: OrderSide, volume: Option<f64>, price: Option<f64>, ord_type: OrderType, identifier: Option<&str>) -> Result<Response, ResponseError> {
        let mut url = Url::parse(&format!("{URL_SERVER}{URL_ORDER}")).unwrap();
        
        url.query_pairs_mut()
            .append_pair("market", market_id)
            .append_pair("side", &side.to_string())
            .append_pair("ord_type", &ord_type.to_string());
            
        if let Some(price) = price {
            let price = format!("{:.8}", price);
            url.query_pairs_mut().append_pair("price", &price.to_string());
        }

        if let Some(volume) = volume {
            let volume = format!("{:.8}", volume);
            url.query_pairs_mut().append_pair("volume", &volume.to_string());
        }

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
}
