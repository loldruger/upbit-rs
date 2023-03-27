use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Url, Response};

use super::{
    super::{
        URL_ORDER, URL_SERVER,
        constant::{OrdSide, OrdType},
        response::{OrderInfo, ResponseErrorSource},
        response_source::{OrderInfoSource}
    },
    request::RequestWithQuery
};

impl OrderInfo {
    pub async fn order(market_id: &str, side: OrdSide, volume: Option<f64>, price: Option<f64>, ord_type: OrdType, identifier: Option<&str>) -> Result<Self, ResponseErrorSource> {
        let res = Self::request_order(market_id, side, volume, price, ord_type, identifier).await;
        let res_serialized = res.text().await.unwrap();
        let res_deserialized = serde_json::from_str(&res_serialized)
            .and_then(|x: OrderInfoSource| {
                let res = OrderInfo {
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
                };
                
                Ok(res)
            })
            .map_err(|_| {
                let res_deserialized_error: ResponseErrorSource = serde_json::from_str(&res_serialized)
                    .and_then(|e: ResponseErrorSource| {
                        Ok(e)
                    })
                    .unwrap();

                res_deserialized_error
            });

        res_deserialized
    }

    async fn request_order(market_id: &str, side: OrdSide, volume: Option<f64>, price: Option<f64>, ord_type: OrdType, identifier: Option<&str>) -> Response {
        let mut url = Url::parse(&format!("{URL_SERVER}{URL_ORDER}")).unwrap();
        
        url.query_pairs_mut()
        .append_pair("market", market_id)
        .append_pair("side", side.into())
        .append_pair("ord_type", ord_type.into());
        
        if price.is_some() {
            let _price = format!("{:.8}", price.unwrap());
            url.query_pairs_mut()
                .append_pair("price", _price.as_str());
        }
        
        if volume.is_some() {
            let _volume = format!("{:.8}", volume.unwrap()); 
            url.query_pairs_mut()
                .append_pair("volume", _volume.as_str());
        }

        if identifier.is_some() {
            url.query_pairs_mut()
                .append_pair("identifier", identifier.unwrap());
        }

        let asdf: Option<String> = if let Some(x) = url.query() {
            let mut y = x.replace("=", ":");
            y = y.replace("&", ",");
            y.insert_str(0, "{");
            y.insert_str(y.len(), "}");
            Some(y)
        } else {
            None
        };

        let token_string = Self::set_token_with_query(url.as_str());
        let res = reqwest::Client::new()
            .post(url.as_str())
            .json(&asdf)
            .header(ACCEPT, "application/json")
            .header(CONTENT_TYPE, "application/json")
            .header(AUTHORIZATION, &token_string)
            .send()
            .await
            .unwrap();

        res
    }
}
