use reqwest::{Response, Url};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};

use crate::request::RequestWithQuery;

use super::{
    super::constant::{URL_DEPOSITS_GENERATE_COIN_ADDRESS, URL_SERVER},
    super::response::{
        CoinAddressGen,
        CoinAddressGenResponse,
        CoinAddressGenFirstResponse,
        CoinAddressGenSecondaryResponse,
        ResponseError,
        ResponseErrorBody,
        ResponseErrorState,
        ResponseErrorSource
    },
};

impl CoinAddressGen {
    pub async fn generate_deposit_address(currency: &str, net_type: Option<&str>) -> Result<Self, ResponseError> {
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
            .map(|x: CoinAddressGenResponse| -> CoinAddressGen {
                Self {
                    response: x
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

    async fn request(currency: &str, net_type: Option<&str>) -> Result<Response, ResponseError> {
        let mut url = Url::parse(&format!("{URL_SERVER}{URL_DEPOSITS_GENERATE_COIN_ADDRESS}?currency={currency}")).unwrap();
        let token_string = Self::set_token_with_query(url.as_str())?;

        if net_type.is_some() {
            url.query_pairs_mut().append_pair("net_type", net_type.unwrap());
        }

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
