use reqwest::header::{ACCEPT, AUTHORIZATION};
use reqwest::{Url, Response};

use crate::request::Request;

use super::{
    super::constant::{URL_DEPOSITS_COIN_ADDRESSES, URL_SERVER},
    super::response::{
        CoinAddressResponse,
        ResponseError,
        ResponseErrorBody,
        ResponseErrorState
    }
};

impl CoinAddressResponse {
    pub async fn list_coin_address_info() -> Result<Vec<Self>, ResponseError> {
        let res = Self::request_list().await?;
        let res_serialized = res.text().await.unwrap();

        if res_serialized.contains("error") {
            return Err(serde_json::from_str(&res_serialized)
                .map(crate::response::response_error)                
                .ok()
                .unwrap()
            )
        }
        
        serde_json::from_str(&res_serialized)
            .map(|x: Vec<CoinAddressResponse>| {
                x
                    .into_iter()
                    .map(|x| {
                        Self {
                            currency: x.currency,
                            net_type: x.net_type,
                            deposit_address: x.deposit_address,
                            secondary_address: x.secondary_address,
                        }
                    })
                    .collect::<Vec<Self>>()
            })
            .map_err(crate::response::response_error_from_json)
    }

    async fn request_list() -> Result<Response, ResponseError> {
        let url = Url::parse(&format!("{URL_SERVER}{URL_DEPOSITS_COIN_ADDRESSES}")).unwrap();
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
