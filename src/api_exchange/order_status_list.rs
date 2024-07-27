use reqwest::{
    header::{ACCEPT, AUTHORIZATION},
    Response, 
    Url
};

use crate::request::Request;

use super::{
    super::constant::{URL_ORDER_STATUS_LIST, URL_SERVER},
    super::response::{
        OrderInfo,
        OrderInfoSource,
        ResponseError
    }
};

impl OrderInfo {
    pub async fn get_order_state_list() -> Result<Vec<Self>, ResponseError> {
        let res = Self::request().await?;
        let res_serialized = res.text().await.unwrap();
        
        if res_serialized.contains("error") {
            return Err(serde_json::from_str(&res_serialized)
                .map(crate::response::response_error)                
                .ok()
                .unwrap()
            )
        }

        serde_json::from_str(&res_serialized)
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
                            trades_count: x.trades_count()
                        })
                    .collect::<Vec<Self>>()
            })
            .map_err(crate::response::response_error_from_json)
    }

    async fn request() -> Result<Response, ResponseError> {
        let url = Url::parse(&format!("{URL_SERVER}{URL_ORDER_STATUS_LIST}")).unwrap();
        let token_string = Self::set_token()?;

        reqwest::Client::new()
            .get(url.as_str())
            .header(ACCEPT, "application/json")
            .header(AUTHORIZATION, &token_string)
            .send()
            .await
            .map_err(crate::response::response_error_from_reqwest)
    }
}
