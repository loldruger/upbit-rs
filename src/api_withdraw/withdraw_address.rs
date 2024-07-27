use reqwest::{
    header::{ACCEPT, AUTHORIZATION},
    Response, 
    Url
};

use crate::request::Request;

use super::{
    super::constant::{URL_WITHDRAWS_COIN_ADDRESS, URL_SERVER},
    super::response::{
        WithdrawCoinAddress,
        ResponseError,
        ResponseErrorBody,
        ResponseErrorState
    }
};

impl WithdrawCoinAddress {
    pub async fn get_withdraw_address() -> Result<Self, ResponseError> {
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
            .map(|x: WithdrawCoinAddress| {
                Self {
                    currency: x.currency,
                    net_type: x.net_type,
                    network_name: x.network_name,
                    withdraw_address: x.withdraw_address,
                    secondary_address: x.secondary_address,
                }
            })
            .map_err(crate::response::response_error_from_json)
    }

    async fn request() -> Result<Response, ResponseError> {
        let url = Url::parse(&format!("{URL_SERVER}{URL_WITHDRAWS_COIN_ADDRESS}")).unwrap();
        let token_string = Self::set_token()?;

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
