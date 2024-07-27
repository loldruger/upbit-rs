use reqwest::header::{ACCEPT, AUTHORIZATION};
use reqwest::{Url, Response};

use crate::request::RequestWithQuery;

use super::{
    super::constant::{URL_DEPOSITS_COIN_ADDRESS, URL_SERVER},
    super::response::{
        CoinAddressResponse,
        ResponseError,
        ResponseErrorBody,
        ResponseErrorState,
        ResponseErrorSource
    }
};

impl CoinAddressResponse {
    pub async fn get_coin_address_info(currency: &str, net_type: &str) -> Result<Self, ResponseError> {
        let res = Self::request(currency, net_type).await?;
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
            .map(|x: CoinAddressResponse| {
                Self {
                    currency: x.currency,
                    net_type: x.net_type,
                    deposit_address: x.deposit_address,
                    secondary_address: x.secondary_address,
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

    async fn request(currency: &str, net_type: &str) -> Result<Response, ResponseError> {
        let mut url = Url::parse(&format!("{URL_SERVER}{URL_DEPOSITS_COIN_ADDRESS}")).unwrap();

        url.query_pairs_mut()
            .append_pair("currency", currency)
            .append_pair("net_type", net_type);
        
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
