use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ResponseErrorBodySource {
    pub name: String,
    pub message: String,
}