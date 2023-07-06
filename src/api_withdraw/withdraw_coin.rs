use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Url, Response};

use crate::request::RequestWithQuery;

use super::TransactionType;
use super::{
    super::{
        constant::{URL_WITHDRAWS_COIN, URL_SERVER},
        response::{
            WithdrawInfoDerived,
            WithdrawInfoDerivedSource,
            ResponseError,
            ResponseErrorBody,
            ResponseErrorState,
            ResponseErrorSource
        }
    }
};

impl RequestWithQuery for WithdrawInfoDerived {}
impl WithdrawInfoDerived {
    pub async fn withdraw_coin(
        currency: &str,
        net_type: &str,
        amount: f64,
        address: &str,
        secondary_address: Option<&str>,
        transaction_type: TransactionType
    ) -> Result<Self, ResponseError> {
        let res = Self::request_withdraw_coin(currency, net_type, amount, address, secondary_address, transaction_type).await?;
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
                }).ok().unwrap())
        }

        serde_json::from_str(&res_serialized)
            .map(|x: WithdrawInfoDerivedSource| {
                Self {
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
                    krw_amount: x.krw_amount(),
                    transaction_type: x.transaction_type(),
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

    async fn request_withdraw_coin(
        currency: &str,
        net_type: &str,
        amount: f64,
        address: &str,
        secondary_address: Option<&str>,
        transaction_type: TransactionType
    ) -> Result<Response, ResponseError> {
        let mut url = Url::parse(&format!("{URL_SERVER}{URL_WITHDRAWS_COIN}")).unwrap();
        
        url.query_pairs_mut()
            .append_pair("net_type", net_type)
            .append_pair("currency", currency)
            .append_pair("amount", &format!("{amount}"))
            .append_pair("address", address);
            
        // if price.is_some() {
        //     let price = format!("{:.8}", price.unwrap());
        //     url.query_pairs_mut().append_pair("price", price.as_str());
        // }
        
        // if volume.is_some() {
        //     let volume = format!("{:.8}", volume.unwrap()); 
        //     url.query_pairs_mut().append_pair("volume", volume.as_str());
        // }

        // if identifier.is_some() {
        //     url.query_pairs_mut().append_pair("identifier", identifier.unwrap());
        // }

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
            // .header(CONTENT_TYPE, "application/json")
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
