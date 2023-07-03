use reqwest::{
    header::{ACCEPT, AUTHORIZATION},
    Response, 
    Url
};

use super::{
    super::constant::{URL_ORDER_STATE_LIST, URL_SERVER},
    super::response::{OrderInfo},
    super::response_source::{OrderInfoSource, ResponseErrorSource},
    request::Request
};

impl OrderInfo {
    pub async fn get_order_state_list() -> Result<Vec<Self>, ResponseErrorSource> {
        let res = Self::request().await;
        let res_serialized = res.text().await.unwrap();
        
        serde_json::from_str(&res_serialized)
            .map(|x: Vec<OrderInfoSource>| {
                x
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
            }).map_err(|_| serde_json::from_str(&res_serialized).unwrap())
    }

    async fn request() -> Response {
        let url = Url::parse(&format!("{URL_SERVER}{URL_ORDER_STATE_LIST}")).unwrap();
        let token_string = Self::set_token();
        let client = reqwest::Client::new();
        
        client
            .get(url.as_str())
            .header(ACCEPT, "application/json")
            .header(AUTHORIZATION, &token_string)
            .send()
            .await
            .unwrap()
    }
}
