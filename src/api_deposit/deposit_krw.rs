use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Url, Response};

use crate::constant::TwoFactorType;

use super::{
    super::{
        constant::{URL_WITHDRAWS_KRW, URL_SERVER},
        request::RequestWithQuery,
        response::{
            WithdrawalDepositInfo,
            WithdrawalDepositInfoSource,
            ResponseError,
            ResponseErrorBody,
            ResponseErrorState,
            ResponseErrorSource
        }
    }
};

impl WithdrawalDepositInfo {
    pub async fn deposit_krw(amount: f64, two_factor_type: TwoFactorType) -> Result<Self, ResponseError> {
        let res = Self::request_deposit_krw(amount, two_factor_type).await?;
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
            .map(|x: WithdrawalDepositInfoSource| {
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

    async fn request_deposit_krw(amount: f64, two_factor_type: TwoFactorType) -> Result<Response, ResponseError> {
        let mut url = Url::parse(&format!("{URL_SERVER}{URL_WITHDRAWS_KRW}")).unwrap();
        
        url.query_pairs_mut()
            .append_pair("amount", &format!("{amount}"))
            .append_pair("two_factor_type", &two_factor_type.to_string());
            
        let token_string = Self::set_token_with_query(url.as_str())?;
        
        reqwest::Client::new()
            .post(url.as_str())
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
