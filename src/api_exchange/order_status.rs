use reqwest::header::{ACCEPT, AUTHORIZATION};
use reqwest::{Url, Response};

use crate::request::RequestWithQuery;

use super::{
    super::constant::{URL_ORDER_STATUS, URL_SERVER},
    super::response::{
        OrderInfo,
        OrderStatus,
        OrderStatusSource,
        ObjectTrades,
        ResponseError,
        ResponseErrorBody,
        ResponseErrorState,
        ResponseErrorSource
    }
};

impl RequestWithQuery for OrderStatus {}
impl OrderStatus {
    pub async fn get_order_status(uuid: Option<&str>, identifier: Option<&str>) -> Result<Self, ResponseError> {
        if uuid.is_none() && identifier.is_none() {
            return Err(ResponseError {
                state: ResponseErrorState::InternalNeitherParameterSpecified,
                error: ResponseErrorBody {
                    name: "internal_neither_parameter_specified".to_owned(),
                    message: "Either `uuid` or `identifier` parameter must be specified.".to_owned()
                }
            });
        } else if uuid.is_some() && identifier.is_some() {
            return Err(ResponseError {
                state: ResponseErrorState::InternalTooManyParameterSpecified,
                error: ResponseErrorBody {
                    name: "invalid_parameter_combination".to_owned(),
                    message: "You can specify either a 'uuid' or an 'identifier', but not both.".to_owned()                
                }
            });
        }

        let res = Self::request(uuid, identifier).await?;
        let res_serialized = res.text().await.unwrap();

        if res_serialized.contains("error") {
            return Err(serde_json::from_str(&res_serialized)
                .map(|e: ResponseErrorSource| {
                    ResponseError {
                        state: ResponseErrorState::from(e.error.name.as_str()),
                        error: ResponseErrorBody {
                            name: e.error.name,
                            message: e.error.message
                        },
                    }
                })
                .ok()
                .unwrap()
            )
        }
        
        serde_json::from_str(&res_serialized)
            .map(|x: OrderStatusSource| {
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
            .map_err(|x| {
                ResponseError {
                    state: ResponseErrorState::InternalJsonParseError,
                    error: ResponseErrorBody {
                        name: "internal_json_parse_error".to_owned(),
                        message: x.to_string()
                    },
                }
            })
    }

    async fn request(uuid: Option<&str>, identifier: Option<&str>) -> Result<Response, ResponseError> {
        let mut url = Url::parse(&format!("{URL_SERVER}{URL_ORDER_STATUS}")).unwrap();

        if uuid.is_some() {
            url.query_pairs_mut().append_pair("uuid", uuid.unwrap());
        }

        if identifier.is_some() {
            url.query_pairs_mut().append_pair("identifier", identifier.unwrap());
        }

        let token_string = Self::set_token_with_query(url.as_str())?;

        reqwest::Client::new()
            .get(url.as_str())
            .header(ACCEPT, "application/json")
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
