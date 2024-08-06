use jsonwebtokens as jwt;
use jwt::{Algorithm, AlgorithmID};
use reqwest::Url;
use serde_json::json;
use sha2::{Digest, Sha512};
use uuid::Uuid;

use crate::response::{
    response_error_internal_hmac_error, response_error_internal_token_encode_error, ResponseError,
};

pub trait Request {
    fn set_token() -> Result<String, ResponseError> {
        let access_key = envmnt::get_or_panic("ACCESS_KEY");
        let secret_key = envmnt::get_or_panic("SECRET_KEY");
        let alg = Algorithm::new_hmac(AlgorithmID::HS256, secret_key)
            .map_err(response_error_internal_hmac_error)?;

        let header = json!({
            "alg": alg.name()
        });

        let payload = json!({
            "access_key": access_key,
            "nonce": Uuid::new_v4(),
        });

        let token = jwt::encode(&header, &payload, &alg)
            .map_err(response_error_internal_token_encode_error)?;

        Ok(format!("Bearer {token}"))
    }
}

pub trait RequestWithQuery {
    fn set_token_with_query(url: &str) -> Result<String, ResponseError> {
        let access_key = envmnt::get_or_panic("ACCESS_KEY");
        let secret_key = envmnt::get_or_panic("SECRET_KEY");
        let url =
            Url::parse(url).map_err(crate::response::response_error_internal_url_parse_error)?;
        let url_parsed = url.query().unwrap_or("");

        let mut hasher = Sha512::new();
        hasher.update(url_parsed.as_bytes());

        let hasher_hex = format!("{:x}", hasher.finalize());
        let alg = Algorithm::new_hmac(AlgorithmID::HS256, secret_key)
            .map_err(response_error_internal_hmac_error)?;

        let header = json!({
            "alg": alg.name()
        });

        let payload = json!({
            "access_key": access_key,
            "nonce": Uuid::new_v4(),
            "query_hash": hasher_hex,
            "query_hash_alg": "SHA512",
        });

        let token = jwt::encode(&header, &payload, &alg)
            .map_err(response_error_internal_token_encode_error)?;

        Ok(format!("Bearer {token}"))
    }
}
