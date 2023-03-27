use serde::Deserialize;
use super::super::response_source::ResponseErrorBodySource;

#[derive(Deserialize, Debug)]
pub struct ResponseErrorSource {
    pub error: ResponseErrorBodySource
}
#[derive(Deserialize, Debug)]
pub enum ResponseErrorState {
    JwtVerificationError, //"Failed to verify Jwt token."
    ExpiredAccessKey,
    InvalidQueryPayload,
    CreateAskError,
    CreateBidError,
    InvalidAccessKey,
    InvalidParameter,
    InvalideVolumeBid, //"주문수량 단위를 잘못 입력하셨습니다. 확인 후 시도해주세요."
    InvalidPriceBid, //"주문가격 단위를 잘못 입력하셨습니다. 확인 후 시도해주세요."
    OrderNotFound,
    UnderMinTotalAsk, //"최소주문금액 이상으로 주문해주세요"
    UnderMinTotalBid, //"최소주문금액 이상으로 주문해주세요"
    InsufficientFundsAsk,
    InsufficientFundsBid,
    NonceUsed,
    NoAuthorizationIp,
    OutOfScope,
    WithdrawAddressNotRegisterd,
    NotSupportedOrdType, //"현재 해당 마켓에서 지원하지 않는 주문입니다. 주문 조건을 다시 확인해주시기 바랍니다."
    UnexpectedError
}

impl From<&str> for ResponseErrorState {
    fn from(value: &str) -> Self {
        match value {
            "internal_invalid_parameter" => ResponseErrorState::InvalidParameter,
            "jwt_verification" => ResponseErrorState::JwtVerificationError,
            "expired_access_key" => ResponseErrorState::ExpiredAccessKey,
            "invalid_query_payload" => ResponseErrorState::InvalidQueryPayload,
            "invalid_access_key" => ResponseErrorState::InvalidAccessKey,
            "invalid_volume_bid" => ResponseErrorState::InvalideVolumeBid,
            "invalid_price_bid" => ResponseErrorState::InvalidPriceBid,
            "under_min_total_ask" => ResponseErrorState::UnderMinTotalAsk,
            "under_min_total_bid" => ResponseErrorState::UnderMinTotalBid,
            "insufficient_funds_ask" => ResponseErrorState::InsufficientFundsAsk,
            "insufficient_funds_bid" => ResponseErrorState::InsufficientFundsBid,
            "create_ask_error" => ResponseErrorState::CreateAskError,
            "create_bid_error" => ResponseErrorState::CreateBidError,
            "nonce_used" => ResponseErrorState::NonceUsed,
            "no_authorization_i_p" => ResponseErrorState::NoAuthorizationIp,
            "out_of_scope" => ResponseErrorState::OutOfScope,
            "withdraw_address_not_registerd" => ResponseErrorState::WithdrawAddressNotRegisterd,
            "order_not_found" => ResponseErrorState::OrderNotFound,
            "not_supported_ord_type" => ResponseErrorState::NotSupportedOrdType, 
            "server_error" => ResponseErrorState::UnexpectedError,
            _ => ResponseErrorState::UnexpectedError
        }
    }
}