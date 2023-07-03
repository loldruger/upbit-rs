use reqwest::header::{ACCEPT, AUTHORIZATION};
use reqwest::{Url, Response};

use super::{
    super::constant::{URL_ORDER_STATE, URL_SERVER},
    super::response::{
        OrderInfo,
        OrderState,
        ObjectTrades
    },
    super::response_source::{
        ResponseErrorBodySource,
        ResponseErrorSource,
        OrderStateSource
    },
    request::RequestWithQuery,
};

impl RequestWithQuery for OrderState {}
impl OrderState {
    pub async fn get_order_state(uuid: Option<&str>, identifier: Option<&str>) -> Result<Self, ResponseErrorSource> {
        if uuid.is_none() && identifier.is_none() {
            return Err(ResponseErrorSource {
                error: ResponseErrorBodySource {
                    name: "internal_invalid_parameter".to_owned(),
                    message: "either parameter uuid or identifier must be specified.".to_owned()
                }
            });
        } else if uuid.is_some() && identifier.is_some() {
            return Err(ResponseErrorSource {
                error: ResponseErrorBodySource {
                    name: "internal_invalid_parameter".to_owned(),
                    message: "only one parameter of uuid and identifier must be specified.".to_owned()
                }
            });
        }

        let res = Self::request(uuid, identifier).await?;
        let res_serialized = res.text().await.unwrap();

        serde_json::from_str(&res_serialized)
            .map(|x: OrderStateSource| {
                Self {
                    order_info: OrderInfo {
                        uuid: x.order_info.uuid(),
                        side: x.order_info.side(),
                        ord_type: x.order_info.ord_type(),
                        price: x.order_info.price(),
                        state: x.order_info.state(),
                        market: x.order_info.market(),
                        created_at: x.order_info.created_at(),
                        volume: x.order_info.volume(),
                        remaining_volume: x.order_info.remaining_volume(),
                        reserved_fee: x.order_info.reserved_fee(),
                        remaining_fee: x.order_info.remaining_fee(),
                        paid_fee: x.order_info.paid_fee(),
                        locked: x.order_info.locked(),
                        executed_volume: x.order_info.executed_volume(),
                        trades_count: x.order_info.trades_count(),
                    },
                    trades: x.trades
                        .into_iter()
                        .map(|object_trades| ObjectTrades {
                            market: object_trades.market,
                            uuid: object_trades.uuid,
                            price: object_trades.price.parse().unwrap_or(0.0),
                            volume: object_trades.volume.parse().unwrap_or(0.0),
                            funds: object_trades.funds.parse().unwrap_or(0.0),
                            side: object_trades.side,
                            created_at: object_trades.created_at,
                        })
                        .collect(),
                }
            })
            .map_err(|_| serde_json::from_str(&res_serialized).unwrap())
    }

    async fn request(uuid: Option<&str>, identifier: Option<&str>) -> Result<Response, ResponseErrorSource> {
        let mut url = Url::parse(&format!("{URL_SERVER}{URL_ORDER_STATE}")).unwrap();

        if uuid.is_some() {
            url.query_pairs_mut().append_pair("uuid", uuid.unwrap());
        }

        if identifier.is_some() {
            url.query_pairs_mut().append_pair("identifier", identifier.unwrap());
        }

        let token_string = Self::set_token_with_query(url.as_str())?;
        let client = reqwest::Client::new();
        
        client
            .get(url.as_str())
            .header(ACCEPT, "application/json")
            .header(AUTHORIZATION, &token_string)
            .send()
            .await
            .map_err(|x| {
                ResponseErrorSource {
                    error: ResponseErrorBodySource {
                        name: "internal_reqwest_error".to_owned(),
                        message: x.to_string()
                    }
                }
            })
    }
}
