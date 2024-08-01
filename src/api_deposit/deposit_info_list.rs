use reqwest::{
    header::{ACCEPT, AUTHORIZATION},
    Response, 
    Url
};
use crate::request::RequestWithQuery;

use super::{
    super::constant::{
        URL_DEPOSITS,
        URL_SERVER,
        OrderBy
    },
    super::response::{
        TransactionInfo,
        TransactionInfoSource,
        ResponseError,
    }, DepositState,
};

impl TransactionInfo {
    pub async fn inquiry_deposit_list(
        currency: &str,
        state: DepositState,
        uuids: Option<&[&str]>,
        txids: Option<&[&str]>,
        limit: u32,
        page: u32,
        order_by: OrderBy
    ) -> Result<Vec<Self>, ResponseError> {
        let res = Self::request_deposit_list(currency, state, uuids, txids, limit, page, order_by).await?;
        let res_serialized = res.text().await.map_err(crate::response::response_error_from_reqwest)?;
        
        if res_serialized.contains("error") {
            return Err(serde_json::from_str(&res_serialized)
                .map(crate::response::response_error)                
                .ok()
                .unwrap()
            )
        }

        serde_json::from_str(&res_serialized)
            .map(|x: Vec<TransactionInfoSource>| {
                x
                    .into_iter()
                    .map(|x| Self {
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
                    })
                    .collect::<Vec<Self>>()
            })
            .map_err(crate::response::response_error_from_json)
    }

    async fn request_deposit_list(
        currency: &str,
        state: DepositState,
        uuids: Option<&[&str]>,
        txids: Option<&[&str]>,
        limit: u32,
        page: u32,
        order_by: OrderBy
    ) -> Result<Response, ResponseError> {
        let mut url = Url::parse(&format!("{URL_SERVER}{URL_DEPOSITS}")).unwrap();
        
        url.query_pairs_mut()
            .append_pair("currency", currency)
            .append_pair("state", &state.to_string())
            .append_pair("limit", &format!("{limit}"))
            .append_pair("page", &format!("{page}"))
            .append_pair("order_by", &order_by.to_string());
    
        let url = if let Some(uuids) = uuids {
            for uuid in uuids {
                url.query_pairs_mut().append_pair("uuids", uuid);
            }

            url.as_str().replace("uuids", "uuids[]")

        } else if let Some(txids) = txids {
            for txid in txids {
                url.query_pairs_mut().append_pair("txids", txid);
            }

            url.as_str().replace("txids", "txids[]")

        } else {
            url.as_str().to_string()
        };

        let token_string = Self::set_token_with_query(&url)?;

        reqwest::Client::new()
            .get(url)
            .header(ACCEPT, "application/json")
            .header(AUTHORIZATION, &token_string)
            .send()
            .await
            .map_err(crate::response::response_error_from_reqwest)
    }
}
