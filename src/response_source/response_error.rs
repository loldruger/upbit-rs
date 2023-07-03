use serde::Deserialize;

use crate::response::ResponseErrorState;

#[derive(Deserialize, Debug)]
pub struct ResponseError {
    pub state: ResponseErrorState,
    pub error: ResponseErrorBody
}

#[derive(Deserialize, Debug)]
pub struct ResponseErrorBody {
    pub name: String,
    pub message: String,
}