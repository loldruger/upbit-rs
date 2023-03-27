use reqwest::Response;
use reqwest::header::{ACCEPT, AUTHORIZATION};

use super::{
    super::constant::{URL_ACCOUNTS, URL_SERVER},
    super::response::{AccountsInfo},
    super::response_source::{AccountsInfoSource, ResponseErrorSource},
    request::Request
};

impl AccountsInfo {
    pub async fn get_account_info() -> Result<Vec<Self>, ResponseErrorSource> {
        let res = Self::request().await;
        let res_serialized = res.text().await.unwrap();
        let res_deserialized = serde_json::from_str(&res_serialized)
            .and_then(|x: Vec<AccountsInfoSource>| {
                let res = x
                    .into_iter()
                    .map(|x| Self {
                        currency: x.currency(),
                        balance: x.balance(),
                        locked: x.locked(),
                        avg_buy_price: x.avg_buy_price(),
                        avg_buy_price_modified: x.avg_buy_price_modified(),
                        unit_currency: x.unit_currency()
                    })
                    .collect::<Vec<Self>>();
                
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

    async fn request() -> Response {
        let token_string = Self::set_token();
        let res = reqwest::Client::new()
            .get(format!("{URL_SERVER}{URL_ACCOUNTS}"))
            .header(ACCEPT, "application/json")
            .header(AUTHORIZATION, &token_string)
            .send()
            .await
            .unwrap();

        res
    }
}
