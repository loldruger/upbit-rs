use serde::Deserialize;
use serde_json::Error;

/// List of response error item
#[derive(Deserialize, Debug)]
pub enum ResponseErrorState {
    /// either parameter uuid or identifier must be specified.
    InternalNeitherParameterSpecified,
    /// only one parameter of uuid and identifier must be specified.
    InternalTooManyParameterSpecified, 
    /// "internal_reqwest_error"
    InternalReqwestError,
    /// "internal_hmac_error"
    InternalHmacError,
    /// "internal_token_encode_error"
    InternalTokenEncodeError,
    /// "internal_json_parse_error"
    InternalJsonParseError,
    /// "JWT 헤더의 페이로드가 올바르지 않습니다."
    /// 
    /// "서명에 사용한 페이로드 값을 확인해주세요."
    InvalidQueryPayload,
    /// "잘못된 API 키"
    InvalidAccessKey,
    /// "주문수량 단위를 잘못 입력하셨습니다. 확인 후 시도해주세요."
    InvalideVolumeBid,
    /// "주문가격 단위를 잘못 입력하셨습니다. 확인 후 시도해주세요."
    InvalidPriceBid,
    /// "잘못된 파라미터"
    InvalidParameter,
    /// "주문을 찾지 못했습니다."
    OrderNotFound,
    /// "최소주문금액 이상으로 주문해주세요"
    UnderMinTotalAsk,
    /// "최소주문금액 이상으로 주문해주세요"
    UnderMinTotalBid,
    /// "Failed to verify Jwt token."
    JwtVerificationError,
    /// "주문 요청 정보가 올바르지 않습니다."
    CreateAskError,
    /// "주문 요청 정보가 올바르지 않습니다."
    CreateBidError,
    /// "디지털 자산 지갑정보를 찾지 못했습니다."
    CoinAddressNotFound,
    /// "매도 가능 잔고가 부족합니다."
    InsufficientFundsAsk,
    /// "매수 가능 잔고가 부족합니다."
    InsufficientFundsBid,
    /// "API 키가 만료되었습니다."
    ExpiredAccessKey,
    /// "이미 요청한 nonce값이 다시 사용되었습니다."
    /// 
    /// "JWT 헤더 페이로드의 nonce 값은 매번 새로운 값을 사용해야합니다."
    NonceUsed,
    /// "허용되지 않은 IP 주소입니다."
    NoAuthorizationIp,
    /// "허용되지 않은 기능입니다."
    OutOfScope,
    /// "등록된 출금 주소가 아닙니다."
    WithdrawAddressNotRegisterd, 
    /// "출금 금액이 부족합니다."
    WithdrawInsufficientBalance,
    /// "현재 해당 마켓에서 지원하지 않는 주문입니다. 주문 조건을 다시 확인해주시기 바랍니다."
    NotSupportedOrdType,
    /// "not found market marketId: XXX"
    NotFoundMarket,
    /// "잘못된 API 요청입니다"
    /// 
    /// "누락된 파라미터가 없는지 확인해주세요."
    ValidationError,
    /// "서버 에러"
    ServerError,
    /// unhandled error
    UnexpectedError
}

impl From<&str> for ResponseErrorState {
    fn from(value: &str) -> Self {
        match value {
            "internal_neither_parameter_specified" => Self::InternalNeitherParameterSpecified,
            "invalid_parameter_combination" => Self::InternalTooManyParameterSpecified,
            "internal_reqwest_error" => Self::InternalReqwestError,
            "internal_hmac_error" => Self::InternalHmacError,
            "internal_token_encode_error" => Self::InternalTokenEncodeError,
            "jwt_verification" => Self::JwtVerificationError,
            "expired_access_key" => Self::ExpiredAccessKey,
            "invalid_query_payload" => Self::InvalidQueryPayload,
            "invalid_access_key" => Self::InvalidAccessKey,
            "invalid_volume_bid" => Self::InvalideVolumeBid,
            "invalid_price_bid" => Self::InvalidPriceBid,
            "invalid_parameter" => Self::InvalidParameter,
            "under_min_total_ask" => Self::UnderMinTotalAsk,
            "under_min_total_bid" => Self::UnderMinTotalBid,
            "insufficient_funds_ask" => Self::InsufficientFundsAsk,
            "insufficient_funds_bid" => Self::InsufficientFundsBid,
            "coin_address_not_found" => Self::CoinAddressNotFound,
            "create_ask_error" => Self::CreateAskError,
            "create_bid_error" => Self::CreateBidError,
            "nonce_used" => Self::NonceUsed,
            "no_authorization_i_p" => Self::NoAuthorizationIp,
            "out_of_scope" => Self::OutOfScope,
            "withdraw_address_not_registered" => Self::WithdrawAddressNotRegisterd,
            "withdraw_insufficient_balance" => Self::WithdrawInsufficientBalance,
            "order_not_found" => Self::OrderNotFound,
            "not_supported_ord_type" => Self::NotSupportedOrdType,
            "notfoundmarket" => Self::NotFoundMarket,
            "validation_error" => Self::ValidationError,
            "server_error" => Self::ServerError,
            _ => Self::UnexpectedError
        }
    }
}

/// Derived Response error data
#[derive(Deserialize, Debug)]
pub struct ResponseError {
    pub state: ResponseErrorState,
    pub error: ResponseErrorBody
}

/// Original error data structure
#[derive(Deserialize, Debug)]
pub struct ResponseErrorSource {
    pub error: ResponseErrorBody
}

/// Error body
#[derive(Deserialize, Debug)]
pub struct ResponseErrorBody {
    pub name: String,
    pub message: String,
}

pub fn response_error(e: ResponseErrorSource) -> ResponseError {
    ResponseError {
        state: ResponseErrorState::from(e.error.name.as_str()),
        error: ResponseErrorBody {
            name: e.error.name,
            message: e.error.message
        },
    }
}

pub fn response_error_from_json(e: Error) -> ResponseError {
    ResponseError {
        state: ResponseErrorState::InternalJsonParseError,
        error: ResponseErrorBody {
            name: "internal_json_parse_error".to_owned(),
            message: e.to_string()
        },
    }
}

pub fn response_error_from_reqwest(e: reqwest::Error) -> ResponseError {
    ResponseError {
        state: ResponseErrorState::InternalReqwestError,
        error: ResponseErrorBody {
            name: "internal_reqwest_error".to_owned(),
            message: e.to_string()
        },
    }
}