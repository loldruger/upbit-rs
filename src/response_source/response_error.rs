use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ResponseErrorSource {
    pub error: ResponseErrorBodySource
}

#[derive(Deserialize, Debug)]
pub struct ResponseErrorBodySource {
    pub name: String,
    pub message: String,
}