use reqwest::{
    header::{ACCEPT, AUTHORIZATION},
    Response, 
    Url
};
use crate::request::RequestWithQuery;

use super::{
    super::constant::{
        URL_WITHDRAWS,
        URL_SERVER,
        OrderBy
    },
    super::response::{
        WithdrawInfo,
        WithdrawInfoSource,
        ResponseError,
        ResponseErrorBody,
        ResponseErrorState,
        ResponseErrorSource
    }, WithdrawState,
};
impl RequestWithQuery for WithdrawInfo {}
impl WithdrawInfo {
    pub async fn inquiry_withdraw_list(
        currency: &str,
        state: WithdrawState,
        uuids: Option<&[&str]>,
        txids: Option<&[&str]>,
        limit: u32,
        page: u32,
        order_by: OrderBy
    ) -> Result<Vec<Self>, ResponseError> {
        let res = Self::request(currency, state, uuids, txids, limit, page, order_by).await?;
        let mut res_serialized = res.text().await.unwrap();
        
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
                }).ok().unwrap())
        }

        res_serialized = res_serialized.replace("null", "\"null\"");

        serde_json::from_str(&res_serialized)
            .map(|x: Vec<WithdrawInfoSource>| {
                x
                    .into_iter()
                    .map(|x| Self {
                        r#type: x.r#type(),
                        uuid: x.uuid(),
                        currency: x.currency(),
                        net_type: x.net_type(),
                        txid: x.txid(),
                        state: x.state(),
                        created_at: x.created_at(),
                        done_at: x.done_at(),
                        amount: x.amount(),
                        fee: x.fee(),
                        transaction_type: x.transaction_type(),
                    })
                    .collect::<Vec<Self>>()
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

    async fn request(
        currency: &str,
        state: WithdrawState,
        uuids: Option<&[&str]>,
        txids: Option<&[&str]>,
        limit: u32,
        page: u32,
        order_by: OrderBy
    ) -> Result<Response, ResponseError> {
        let mut url = Url::parse(&format!("{URL_SERVER}{URL_WITHDRAWS}")).unwrap();
        
        url.query_pairs_mut()
            .append_pair("currency", currency)
            .append_pair("state", &state.to_string())
            .append_pair("limit", &format!("{limit}"))
            .append_pair("page", &format!("{page}"))
            .append_pair("order_by", &order_by.to_string());
    
        if uuids.is_some() {
            let uuids = uuids
                .unwrap()
                .join("&")
                .split_inclusive('&')
                .map(|x| format!("uuids[]={x}"))
                .collect::<String>();
            url = Url::parse(&format!("{}&{}", url.as_str(), uuids)).unwrap();
            }

        if txids.is_some() {
            let txids = uuids
                .unwrap()
                .join("&")
                .split_inclusive("&")
                .map(|x| format!("txids[]={x}"))
                .collect::<String>();
            url = Url::parse(&format!("{}&{}", url.as_str(), txids)).unwrap();
        }

        let token_string = Self::set_token_with_query(url.as_str())?;

        reqwest::Client::new()
            .get(url)
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
                    },
                }
            })
    }
}
