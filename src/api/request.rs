use jsonwebtokens as jwt;
use jwt::{Algorithm, AlgorithmID};
use reqwest::Url;
use serde_json::json;
use sha2::{Digest, Sha512};
use uuid::Uuid;

use crate::response_source::{ResponseErrorSource, ResponseErrorBodySource};

pub trait Request {
    fn set_token() -> Result<String, ResponseErrorSource> {
        let access_key = envmnt::get_or_panic("ACCESS_KEY");
        let secret_key = envmnt::get_or_panic("SECRET_KEY");
        let alg = Algorithm::new_hmac(AlgorithmID::HS256, secret_key).unwrap();

        let header = json!({ 
            "alg": alg.name()
        });
        
        let payload = json!({
            "access_key": access_key,
            "nonce": Uuid::new_v4(),
        });

        let token = jwt::encode(&header, &payload, &alg).unwrap();

        Ok(format!("Bearer {token}"))
    }
}

pub trait RequestWithQuery {
    fn set_token_with_query(url: &str) -> Result<String, ResponseErrorSource> {
        let access_key = envmnt::get_or_panic("ACCESS_KEY");
        let secret_key = envmnt::get_or_panic("SECRET_KEY");
        let url = Url::parse(url).ok().unwrap();
        let url_parsed = url.query();

        let mut hasher = Sha512::new();
        hasher.update(url_parsed.unwrap_or("").as_bytes());

        let hasher_hex = format!("{:x}", hasher.finalize());
        let alg = Algorithm::new_hmac(AlgorithmID::HS256, secret_key)
            .map_err(|error| {
                ResponseErrorSource {
                    error: ResponseErrorBodySource {
                        name: "internal_hmac_error".to_owned(),
                        message: error.to_string()
                    }
                }
            })?;
        let header = json!({ "alg": alg.name() });
        let payload = json!({
            "access_key": access_key,
            "nonce": Uuid::new_v4(),
            "query_hash": hasher_hex,
            "query_hash_alg": "SHA512",
        });

        let token = jwt::encode(&header, &payload, &alg)
            .map_err(|error| {
                ResponseErrorSource {
                    error: ResponseErrorBodySource {
                        name: "internal_token_encode_error".to_owned(),
                        message: error.to_string()
                    }
                }
            })?;

        Ok(format!("Bearer {token}"))
    }
}