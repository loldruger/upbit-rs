use reqwest::{Url, Response};
use reqwest::header::ACCEPT;
use serde::Deserialize;

use super::super::{URL_SERVER, URL_MARKET_STATE};
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
        let res_deserialized = serde_json::from_str(&res_serialized)
            .and_then(|x: Vec<MarketStateSource>| {
                let res = x
                    .into_iter()
                    .map(|i| {
                        Self {
                            market: i.market,
                            korean_name: i.korean_name,
                            english_name: i.english_name,
                            market_warning: i.market_warning.and_then(|s| {
                                if s.contains("NONE") {Some(false)} else {Some(true)}
                            })
                        }
                    })
                    .collect();

                Ok(res)
            })
            .map_err(|_| {
                let res_deserialized_error: ResponseErrorSource = serde_json::from_str(&res_serialized)
                    .and_then(|e: ResponseErrorSource| {
                        Ok(e)
                    })
                    .unwrap();

                res_deserialized_error
            });

        res_deserialized
    }

    async fn request(is_warning_shown: bool) -> Response {
        let mut url = Url::parse(&format!("{URL_SERVER}{URL_MARKET_STATE}")).unwrap();
        url.query_pairs_mut().append_pair("isDetails", is_warning_shown.to_string().as_str());

        let res = reqwest::Client::new()
            .get(url.as_str())
            .header(ACCEPT, "application/json")
            .send()
            .await
            .unwrap();

        res
    }
}