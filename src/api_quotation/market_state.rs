use reqwest::{Url, Response};
use reqwest::header::ACCEPT;
use serde::Deserialize;

use super::super::constant::{URL_SERVER, URL_MARKET_STATE};
use crate::response_source::ResponseErrorSource;

#[derive(Deserialize, Debug)]
pub struct MarketState {
    market: String,
    korean_name: String,
    english_name: String,
    market_warning: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct MarketStateSource {
    market: String,
    korean_name: String,
    english_name: String,
    market_warning: Option<String>, //NONE, CAUTION
}

impl MarketState {
    pub async fn get_market_state(is_warning_shown: bool) -> Result<Vec<Self>, ResponseErrorSource>  {
        let res = Self::request(is_warning_shown).await;
        let res_serialized = res.text().await.unwrap();
        

        serde_json::from_str(&res_serialized)
            .map(|x: Vec<MarketStateSource>| {
                

                x
                    .into_iter()
                    .map(|i| {
                        Self {
                            market: i.market,
                            korean_name: i.korean_name,
                            english_name: i.english_name,
                            market_warning: i.market_warning.map(|s| {
                                !s.contains("NONE")
                            })
                        }
                    })
                    .collect()
            })
            .map_err(|_| {
                let res_deserialized_error: ResponseErrorSource = serde_json::from_str(&res_serialized)
                    .unwrap();

                res_deserialized_error
            })
    }

    async fn request(is_warning_shown: bool) -> Response {
        let mut url = Url::parse(&format!("{URL_SERVER}{URL_MARKET_STATE}")).unwrap();
        url.query_pairs_mut().append_pair("isDetails", is_warning_shown.to_string().as_str());

        

        reqwest::Client::new()
            .get(url.as_str())
            .header(ACCEPT, "application/json")
            .send()
            .await
            .unwrap()
    }
}