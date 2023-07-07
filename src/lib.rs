/// Module for exchange APIs
pub mod api_exchange;
/// Module for quotation APIs
pub mod api_quotation;
/// Module for withdrawal APIs
pub mod api_withdraw;
/// Module for deposit APIs
pub mod api_deposit;
/// Set of constants
pub mod constant;
/// Set of concrete request bodies
pub mod request;
/// Set of concrete response bodies
pub mod response;

/// function for setting secret key
pub fn set_secret_key(secret_key: &str) {
    envmnt::set("SECRET_KEY", secret_key);
}

/// function for setting access_key
pub fn set_access_key(access_key: &str) {
    envmnt::set("ACCESS_KEY", access_key);
}