use reqwest::{
    Response,
    Url,
    header::{ACCEPT, AUTHORIZATION}
};

use super::{
    request::RequestWithQuery,
    super::constant::{URL_ORDER_STATE, URL_SERVER},
    super::response::{OrderInfo},
    super::response_source::{OrderInfoSource, ResponseErrorBodySource, ResponseErrorSource}
};

impl OrderInfo {
    pub async fn delete_order(uuid: Option<&str>, identifier: Option<&str>) -> Result<Self, ResponseErrorSource> {
        if uuid.is_none() && identifier.is_none() {
            return Err(ResponseErrorSource {
                error: ResponseErrorBodySource {
                    name: "internal_invalid_parameter".to_owned(),
                    message: "either parameter uuid or identifier must to be specified.".to_owned()
                }
            });
        } else if uuid.is_some() && identifier.is_some() {
            return Err(ResponseErrorSource {
                error: ResponseErrorBodySource {
                    name: "internal_invalid_parameter".to_owned(),
                    message: "only one parameter of uuid and identifier must to be specified.".to_owned()
                }
            });
        }

        let res = Self::request_delete(uuid, identifier).await;
        let res_serialized: String = res.text().await.unwrap();
        
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
            .map_err(|_| serde_json::from_str(&res_serialized).unwrap())
    }

    async fn request_delete(uuid: Option<&str>, identifier: Option<&str>) -> Response {
        let mut url = Url::parse(&format!("{URL_SERVER}{URL_ORDER_STATE}")).unwrap();

        if uuid.is_some() {
            url.query_pairs_mut().append_pair("uuid", uuid.unwrap());
        }

        if identifier.is_some() {
            url.query_pairs_mut()
                .append_pair("identifier", identifier.unwrap());
        }

        let token_string = Self::set_token_with_query(url.as_str());
        let client = reqwest::Client::new();
        

        client
            .delete(url.as_str())
            .header(ACCEPT, "application/json")
            .header(AUTHORIZATION, &token_string)
            .send()
            .await
            .unwrap()
    }
}
