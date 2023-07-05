// use reqwest::Response;
// use reqwest::header::{ACCEPT, AUTHORIZATION};

// use crate::request::Request;
// use crate::response::ResponseErrorState;
// use crate::response_source::ResponseErrorBody;

// use super::{
//     super::constant::{URL_ACCOUNTS, URL_SERVER},
//     super::response::{WithdrawInfo},
//     super::response_source::{WithdrawListSource, ResponseError, ResponseErrorSource},
// };

// impl WithdrawInfo {
//     pub async fn inquiry_withdraw_list() -> Result<Vec<Self>, ResponseError> {
//         let res = Self::request().await?;
//         let res_serialized = res.text().await.unwrap();
        
//         if res_serialized.contains("error") {
//             return Err(serde_json::from_str(&res_serialized)
//                 .map(|e: ResponseErrorSource| {
//                     ResponseError {
//                         state: ResponseErrorState::from(e.error.name.as_str()),
//                         error: ResponseErrorBody {
//                             name: e.error.name,
//                             message: e.error.message
//                         },
//                     }
//                 }).ok().unwrap())
//         }

//         serde_json::from_str(&res_serialized)
//             .map(|x: Vec<WithdrawListSource>| {
//                 x
//                     .into_iter()
//                     .map(|x| Self {
//                         r#type: x.r#type(),
//                         uuid: x.uuid(),
//                         currency: x.currency(),
//                         net_type: x.net_type(),
//                         txid: x.txid(),
//                         state: x.state(),
//                         created_at: x.created_at(),
//                         done_at: x.done_at(),
//                         amount: x.amount(),
//                         fee: x.fee(),
//                         transaction_type: x.transaction_type(),
//                     })
//                     .collect::<Vec<Self>>()
//             })
//             .map_err(|x| {
//                 ResponseError {
//                     state: ResponseErrorState::InternalJsonParseError,
//                     error: ResponseErrorBody {
//                         name: "internal_json_parse_error".to_owned(),
//                         message: x.to_string()
//                     },
//                 }
//             })
//     }

//     async fn request() -> Result<Response, ResponseError> {
//         let token_string = Self::set_token()?;
        
//         reqwest::Client::new()
//             .get(format!("{URL_SERVER}{URL_ACCOUNTS}"))
//             .header(ACCEPT, "application/json")
//             .header(AUTHORIZATION, &token_string)
//             .send()
//             .await
//             .map_err(|x| {
//                 ResponseError {
//                     state: ResponseErrorState::InternalReqwestError,
//                     error: ResponseErrorBody {
//                         name: "internal_reqwest_error".to_owned(),
//                         message: x.to_string()
//                     },
//                 }
//             })
//     }
// }
