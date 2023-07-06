pub mod api_exchange;
pub mod api_quotation;
pub mod api_withdraw;
pub mod constant;
pub mod request;
pub mod response;

/// set secret key
pub fn set_secret_key(secret_key: &str) {
    envmnt::set("SECRET_KEY", secret_key);
}

/// set access_key
pub fn set_access_key(access_key: &str) {
    envmnt::set("ACCESS_KEY", access_key);
}