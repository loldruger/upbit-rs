use reqwest::Response;
use reqwest::header::{ACCEPT, AUTHORIZATION};

use crate::request::Request;

use super::{
    super::constant::{URL_ACCOUNTS, URL_SERVER},
    super::response::{
        AccountsInfo,
        AccountsInfoSource
    },
    super::response::ResponseError
};

impl AccountsInfo {
    pub async fn get_account_info() -> Result<Vec<Self>, ResponseError> {
        let res = Self::request().await?;
        let res_serialized = res.text().await.map_err(crate::response::response_error_from_reqwest)?;

        if res_serialized.contains("error") {
            return Err(serde_json::from_str(&res_serialized)
                .map(crate::response::response_error)                
                .ok()
                .unwrap()
            )
        }

        serde_json::from_str(&res_serialized)
            .map(|i: Vec<AccountsInfoSource>| {
                i
                    .into_iter()
                    .map(|x| Self {
                        currency: x.currency(),
                        balance: x.balance(),
                        locked: x.locked(),
                        avg_buy_price: x.avg_buy_price(),
                        avg_buy_price_modified: x.avg_buy_price_modified(),
                        unit_currency: x.unit_currency()
                    })
                    .collect::<Vec<Self>>()
            })
            .map_err(crate::response::response_error_from_json)
    }

    async fn request() -> Result<Response, ResponseError> {
        let token_string = Self::set_token()?;
        
        reqwest::Client::new()
            .get(format!("{URL_SERVER}{URL_ACCOUNTS}"))
            .header(ACCEPT, "application/json")
            .header(AUTHORIZATION, &token_string)
            .send()
            .await
            .map_err(crate::response::response_error_from_reqwest)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::*;

    #[tokio::test]
    async fn get_account_info() {
        crate::set_access_key(&std::env::var("TEST_ACCESS_KEY").expect("TEST_ACCESS_KEY not set"));
        crate::set_secret_key(&std::env::var("TEST_SECRET_KEY").expect("TEST_SECRET_KEY not set"));

        let res = AccountsInfo::request().await.unwrap();
        let res_serialized = res
            .text()
            .await
            .map_err(crate::response::response_error_from_reqwest)
            .unwrap();

        let json = serde_json::from_str::<Value>(&res_serialized).unwrap();
        let dese = serde_json::from_str(&res_serialized)
            .map(|i: Vec<AccountsInfoSource>| {
                i
                    .into_iter()
                    .map(|x| AccountsInfo {
                        currency: x.currency(),
                        balance: x.balance(),
                        locked: x.locked(),
                        avg_buy_price: x.avg_buy_price(),
                        avg_buy_price_modified: x.avg_buy_price_modified(),
                        unit_currency: x.unit_currency()
                    })
                    .collect::<Vec<AccountsInfo>>()
            })
            .map_err(crate::response::response_error_from_json)
            .unwrap();

        assert_ne!(json.get(0).unwrap().get("currency"), None);
        assert_ne!(json.get(0).unwrap().get("balance"), None);
        assert_ne!(json.get(0).unwrap().get("locked"), None);
        assert_ne!(json.get(0).unwrap().get("avg_buy_price"), None);
        assert_ne!(json.get(0).unwrap().get("avg_buy_price_modified"), None);
        assert_ne!(json.get(0).unwrap().get("unit_currency"), None);
    }
}