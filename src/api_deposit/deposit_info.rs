use reqwest::header::{ACCEPT, AUTHORIZATION};
use reqwest::{Url, Response};

use crate::request::RequestWithQuery;
use crate::response::{TransactionInfo, TransactionInfoSource};

use super::{
    super::constant::{URL_WITHDRAW, URL_SERVER},
    super::response::ResponseError
};

impl TransactionInfo {
    pub async fn get_deposit_info(currency: Option<&str>, uuid: Option<&str>, txid: Option<&str>) -> Result<Self, ResponseError> {
        let res = Self::request_deposit(currency, uuid, txid).await?;
        let res_serialized = res.text().await.unwrap();

        if res_serialized.contains("error") {
            return Err(serde_json::from_str(&res_serialized)
                .map(crate::response::response_error)                
                .ok()
                .unwrap()
            )
        }

        serde_json::from_str(&res_serialized)
            .map(|x: TransactionInfoSource| {
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
            .map_err(crate::response::response_error_from_json)
    }

    async fn request_deposit(currency: Option<&str>, uuid: Option<&str>, txid: Option<&str>) -> Result<Response, ResponseError> {
        let mut url = Url::parse(&format!("{URL_SERVER}{URL_WITHDRAW}")).unwrap();

        if currency.is_some() {
            url.query_pairs_mut().append_pair("currency", currency.unwrap());
        }

        if uuid.is_some() {
            url.query_pairs_mut().append_pair("uuid", uuid.unwrap());
        }

        if txid.is_some() {
            url.query_pairs_mut().append_pair("txid", txid.unwrap());
        }

        let token_string = Self::set_token_with_query(url.as_str())?;

        reqwest::Client::new()
            .get(url.as_str())
            .header(ACCEPT, "application/json")
            .header(AUTHORIZATION, &token_string)
            .send()
            .await
            .map_err(crate::response::response_error_from_reqwest)
    }
}
