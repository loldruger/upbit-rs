mod withdraw_address;
mod withdraw_chance;
mod withdraw_coin;
mod withdraw_info;
mod withdraw_info_list;
mod withdraw_krw;

use core::fmt::Display;

use super::response::{ResponseError, TransactionInfo, TransactionInfoDerived};
use crate::{
    constant::{OrderBy, TransactionType, TwoFactorType},
    response::{WithdrawChance, WithdrawCoinAddress},
};

#[cfg(feature = "sqlx-type")]
use sqlx::Type;

/// List of withdraw state
#[cfg_attr(feature = "sqlx-type", derive(sqlx::Type))]
pub enum WithdrawState {
    /// 대기중
    Waiting,
    /// 진행중
    Processing,
    /// 완료
    Done,
    /// 실패
    Failed,
    /// 취소됨
    Canceled,
    /// 거절됨
    Rejected,
}

impl Display for WithdrawState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WithdrawState::Waiting => write!(f, "WAITING"),
            WithdrawState::Processing => write!(f, "PROCESSING"),
            WithdrawState::Done => write!(f, "DONE"),
            WithdrawState::Failed => write!(f, "FAILED"),
            WithdrawState::Canceled => write!(f, "CANCELED"),
            WithdrawState::Rejected => write!(f, "REJECTED"),
        }
    }
}

impl From<&str> for WithdrawState {
    fn from(value: &str) -> Self {
        match value {
            "waiting" => WithdrawState::Waiting,
            "processing" => WithdrawState::Processing,
            "done" => WithdrawState::Done,
            "failed" => WithdrawState::Failed,
            "canceled" => WithdrawState::Canceled,
            "rejected" => WithdrawState::Rejected,
            a @ _ => panic!("Unexpected value: {}", a),
        }
    }
}

/// 출금 기록을 조회한다. (inquiry the records of withdraws.)
///
/// # Example
/// ```rust
/// use constant::OrderBy;
/// use api_withdraw::WithdrawState;
///
/// // it returns withdraw list of currency "KRW", state "done" ordered by asc
/// let get_witrhdraw_info_list = api_withdraw::get_witrhdraw_info_list("KRW", WithdrawState::Done, None, None, 10, 0, OrderBy::Asc).await;
///
/// // it returns withdraw list of currency "BTC", state "done", txid "98c15999..." ordered by desc
/// let get_witrhdraw_info_list = api_withdraw::get_witrhdraw_info_list(
///     "BTC",
///     WithdrawState::Done,
///     None,
///     Some(&[
///         "98c15999f0bdc4ae0e8a-ed35868bb0c204fe6ec29e4058a3451e-88636d1040f4baddf943274ce37cf9cc",
///     ]),
///         10,
///         0,
///         OrderBy::Desc
///     ).await;
///
/// ```
/// - parameters
/// > `currency` ex) KRW, BTC, ETH etc. <br>
/// > `state`
///  >> *  `WithdrawState::Waiting` 대기중<br>
///  >> *  `WithdrawState::Processing` 진행중<br>
///  >> *  `WithdrawState::Done` 완료<br>
///  >> *  `WithdrawState::Failed` 실패<br>
///  >> *  `WithdrawState::Canceled` 취소됨<br>
///  >> *  `WithdrawState::Rejected` 거절됨<br>
///
/// > `uuids` array of uuid<br>
/// > `txids` array of txid<br>
/// > `limit` pagination limit, max `100`<br>
/// > `page` pagination <br>
/// > `order_by`
///  >> *  `OrderBy::Asc` 오름차순<br>
///  >> *  `OrderBy::Desc` 내림차순<br>
///
/// # Response
/// ```json
/// [
///   {
///     "type": "withdraw",
///     "uuid": "35a4f1dc-1db5-4d6b-89b5-7ec137875956",
///     "currency": "XRP",
///     "txid": "98c15999f0bdc4ae0e8a-ed35868bb0c204fe6ec29e4058a3451e-88636d1040f4baddf943274ce37cf9cc",
///     "state": "DONE",
///     "created_at": "2019-02-28T15:17:51+09:00",
///     "done_at": "2019-02-28T15:22:12+09:00",
///     "amount": "1.00",
///     "fee": "0.0",
///     "transaction_type": "default"
///   }
///   # ....
/// ]
/// ```
///
/// | field                  | description                   | type         |
/// |:-----------------------|:------------------------------|:-------------|
/// | type | 입출금 종류 | String
/// | uuid | 출금의 고유 아이디 | String
/// | currency | 화폐를 의미하는 영문 대문자 코드 | String
/// | net_type | 출금 네트워크 | String
/// | txid | 출금의 트랜잭션 아이디 | String
/// | state | 출금 상태<br> - WAITING : 대기중<br> - PROCESSING : 진행중<br> - DONE : 완료<br> - FAILED : 실패<br> - CANCELLED : 취소됨<br> - REJECTED : 거절됨 | String
/// | created_at | 출금 생성 시간 | DateString
/// | done_at | 출금 완료 시간 | DateString
/// | amount | 출금 금액/수량 | NumberString
/// | fee | 출금 수수료 | NumberString
/// | transaction_type | 출금 유형<br> default : 일반출금<br>internal : 바로출금 | String
pub async fn get_withdraw_info_list(
    currency: &str,
    state: WithdrawState,
    uuids: Option<&[&str]>,
    txids: Option<&[&str]>,
    limit: u32,
    page: u32,
    order_by: OrderBy,
) -> Result<Vec<TransactionInfo>, ResponseError> {
    TransactionInfo::get_withdraw_info_list(currency, state, uuids, txids, limit, page, order_by)
        .await
}

/// 개별 출금 조회.
///
/// # Example
/// ```rust
/// let withdraw_info = api_withdraw::get_withdraw_info(None, Some("9f432943-54e0-40b7-825f-b6fec8b42b79"), None).await;
/// ```
/// - parameters
/// > `currency` ex) KRW, BTC, ETH etc. <br>
/// > `uuid` uuid<br>
/// > `txid` txid<br>
/// # Response
/// ```json
/// [
///   {
///     "type": "withdraw",
///     "uuid": "35a4f1dc-1db5-4d6b-89b5-7ec137875956",
///     "currency": "XRP",
///     "txid": "98c15999f0bdc4ae0e8a-ed35868bb0c204fe6ec29e4058a3451e-88636d1040f4baddf943274ce37cf9cc",
///     "state": "DONE",
///     "created_at": "2019-02-28T15:17:51+09:00",
///     "done_at": "2019-02-28T15:22:12+09:00",
///     "amount": "1.00",
///     "fee": "0.0",
///     "transaction_type": "default"
///   }
///   # ....
/// ]
/// ```
///
/// | field                  | description                   | type         |
/// |:-----------------------|:------------------------------|:-------------|
/// | type | 입출금 종류 | String
/// | uuid | 출금의 고유 아이디 | String
/// | currency | 화폐를 의미하는 영문 대문자 코드 | String
/// | net_type | 출금 네트워크 | String
/// | txid | 출금의 트랜잭션 아이디 | String
/// | state | 출금 상태<br> - WAITING : 대기중<br> - PROCESSING : 진행중<br> - DONE : 완료<br> - FAILED : 실패<br> - CANCELLED : 취소됨<br> - REJECTED : 거절됨 | String
/// | created_at | 출금 생성 시간 | DateString
/// | done_at | 출금 완료 시간 | DateString
/// | amount | 출금 금액/수량 | NumberString
/// | fee | 출금 수수료 | NumberString
/// | transaction_type | 출금 유형<br> default : 일반출금<br>internal : 바로출금 | String
pub async fn get_withdraw_info(
    currency: Option<&str>,
    uuid: Option<&str>,
    txid: Option<&str>,
) -> Result<TransactionInfo, ResponseError> {
    TransactionInfo::get_withdraw_info(currency, uuid, txid).await
}

/// 출금 가능 정보를 조회한다.
///
/// # Example
/// ```rust
/// let withdraw_chance = api_withdraw::get_withdraw_chance("ETH", "ETH").await;
/// ```
/// - parameters
/// > `currency` ex) KRW, BTC, ETH etc. <br>
/// > `uuid` uuid<br>
/// # Response
/// ```json
/// {
///     "member_level": {
///       "security_level": 3,
///       "fee_level": 0,
///       "email_verified": true,
///       "identity_auth_verified": true,
///       "bank_account_verified": true,
///       "kakao_pay_auth_verified": false,
///       "locked": false,
///       "wallet_locked": false
///     },
///     "currency": {
///       "code": "BTC",
///       "withdraw_fee": "0.0005",
///       "is_coin": true,
///       "wallet_state": "working",
///       "wallet_support": [
///         "deposit",
///         "withdraw"
///       ]
///     },
///     "account": {
///       "currency": "BTC",
///       "balance": "10.0",
///       "locked": "0.0",
///       "avg_buy_price": "8042000",
///       "avg_buy_price_modified": false,
///       "unit_currency": "KRW",
///     },
///     "withdraw_limit": {
///       "currency": "BTC",
///       "minimum": null,
///       "onetime": null,
///       "daily": "10.0",
///       "remaining_daily": "10.0",
///       "remaining_daily_krw": "0.0",
///       "fixed": null,
///       "can_withdraw": true
///     }
///   }
/// ```
/// | field                  | description                   | type         |
/// |:-----------------------|:------------------------------|:-------------|
/// | member_level.security_level | 사용자의 보안등급 | Integer |
/// | member_level.fee_level | 사용자의 수수료등급 | Integer |
/// | member_level.email_verified | 사용자의 이메일 인증 여부 | Boolean |
/// | member_level.identity_auth_verified | 사용자의 실명 인증 여부 | Boolean |
/// | member_level.bank_account_verified | 사용자의 계좌 인증 여부 | Boolean |
/// | member_level.kakao_pay_auth_verified | 사용자의 카카오페이 인증 여부 | Boolean |
/// | member_level.locked | 사용자의 계정 보호 상태 | Boolean |
/// | member_level.wallet_locked | 사용자의 출금 보호 상태 | Boolean |
/// | currency.code | 화폐를 의미하는 영문 대문자 코드 | String |
/// | currency.withdraw_fee | 해당 화폐의 출금 수수료 | NumberString |
/// | currency.is_coin | 화폐의 디지털 자산 여부 | Boolean |
/// | currency.wallet_state | 해당 화폐의 지갑 상태 | String |
/// | currency.wallet_support | 해당 화폐가 지원하는 입출금 정보 | Array[String]  |
/// | account.currency | 화폐를 의미하는 영문 대문자 코드 | String |
/// | account.balance | 주문가능 금액/수량 | NumberString |
/// | account.locked | 주문 중 묶여있는 금액/수량 | NumberString |
/// | account.avg_buy_price | 매수평균가 | NumberString |
/// | account.avg_buy_price_modified | 매수평균가 수정 여부 | Boolean |
/// | account.unit_currency | 평단가 기준 화폐 | String | |  |
/// | withdraw_limit.currency | 화폐를 의미하는 영문 대문자 코드 | String |
/// | withdraw_limit.minimum | 출금 최소 금액/수량 | NumberString |
/// | withdraw_limit.onetime | 1회 출금 한도 | NumberString |
/// | withdraw_limit.daily | 1일 출금 한도 | NumberString |
/// | withdraw_limit.remaining_daily | 1일 잔여 출금 한도 | NumberString |
/// | withdraw_limit.remaining_daily_krw | 통합 1일 잔여 출금 한도 | NumberString |
/// | withdraw_limit.fixed | 출금 금액/수량 소수점 자리 수 | Integer |
/// | withdraw_limit.can_withdraw | 출금 지원 여부 | Boolean |
pub async fn get_withdraw_chance(
    currency: &str,
    net_type: &str,
) -> Result<WithdrawChance, ResponseError> {
    WithdrawChance::get_withdraw_chance(currency, net_type).await
}

/// 가상화폐를 출금한다.
///
/// # Example
/// ```rust
/// let withdraw_result_more_info = api_withdraw::withdraw_coin("ETH", "ETH", 0.005, "0x40268F1e99F76b658c6D52d89166EE289EfC225d", None, TransactionType::Default).await;
/// ```
/// - parameters
/// > `currency` ex) KRW, BTC, ETH etc. <br>
/// > `net_type` withdraw network <br>
/// > `amount` amount of withdraw <br>
/// > `address` address registered to withdrawable address<br>
/// > `secondary_address` secondary address <br>
/// > `transaction_type` <br>
/// >> * `TransactionType::Default` 일반출금 <br>
/// >> * `TransactionType::Internal` 바로출금 <br>
/// # Response
/// ```json
/// {
///     "type": "withdraw",
///     "uuid": "9f432943-54e0-40b7-825f-b6fec8b42b79",
///     "currency": "BTC",
///     "txid": "ebe6937b-130e-4066-8ac6-4b0e67f28adc",
///     "state": "processing",
///     "created_at": "2018-04-13T11:24:01+09:00",
///     "done_at": null,
///     "amount": "0.01",
///     "fee": "0.0",
///     "krw_amount": "80420.0", // added info
///     "transaction_type": "default"
/// }
/// ```
/// | field                  | description                   | type         |
/// |:-----------------------|:------------------------------|:-------------|
/// | type| 입출금 종류 | String |
/// | uuid| 출금의 고유 아이디 | String |
/// | currency| 화폐를 의미하는 영문 대문자 코드 | String |
/// | net_type| 출금 네트워크 | String |
/// | txid| 출금의 트랜잭션 아이디 | String |
/// | state| 출금 상태 | String |
/// | created_at| 출금 생성 시간 | DateString |
/// | done_at| 출금 완료 시간 | DateString |
/// | amount| 출금 금액/수량 | NumberString |
/// | fee| 출금 수수료 | NumberString |
/// | krw_amount| 원화 환산 가격 | NumberString |
/// | transaction_type| 출금 유형 | String |
pub async fn withdraw_coin(
    currency: &str,
    net_type: &str,
    amount: f64,
    address: &str,
    secondary_address: Option<&str>,
    transaction_type: TransactionType,
) -> Result<TransactionInfoDerived, ResponseError> {
    TransactionInfoDerived::withdraw_coin(
        currency,
        net_type,
        amount,
        address,
        secondary_address,
        transaction_type,
    )
    .await
}

/// 원화를 출금한다.
///
/// # Example
/// ```rust
/// let withdraw_result_with_kakao_auth = api_withdraw::withdraw_krw(10000.0, api_withdraw::TwoFactorType::Kakao).await;
/// let withdraw_result_with_naver_auth = api_withdraw::withdraw_krw(10000.0, api_withdraw::TwoFactorType::Naver).await;
/// ```
/// - parameters
/// > `amount` amount of withdraw <br>
/// > `two_factor_type`
/// >> * `TwoFactorType::KakaoPay` Two factor identification via kakao <br>
/// >> * `TwoFactorType::Naver` Two factor identification via naver <br>
/// # Response
/// ```json
/// {
///     "type": "withdraw",
///     "uuid": "9f432943-54e0-40b7-825f-b6fec8b42b79",
///     "currency": "KRW",
///     "txid": "ebe6937b-130e-4066-8ac6-4b0e67f28adc",
///     "state": "processing",
///     "created_at": "2018-04-13T11:24:01+09:00",
///     "done_at": null,
///     "amount": "0.01",
///     "fee": "0.0",
///     "transaction_type": "default"
/// }
/// ```
/// | field                  | description                   | type         |
/// |:-----------------------|:------------------------------|:-------------|
/// | type| 입출금 종류 | String |
/// | uuid| 출금의 고유 아이디 | String |
/// | currency| 화폐를 의미하는 영문 대문자 코드 | String |
/// | net_type| 출금 네트워크 | String |
/// | txid| 출금의 트랜잭션 아이디 | String |
/// | state| 출금 상태 | String |
/// | created_at| 출금 생성 시간 | DateString |
/// | done_at| 출금 완료 시간 | DateString |
/// | amount| 출금 금액/수량 | NumberString |
/// | fee| 출금 수수료 | NumberString |
/// | transaction_type| 출금 유형 | String |
pub async fn withdraw_krw(
    amount: f64,
    two_factor_type: TwoFactorType,
) -> Result<TransactionInfo, ResponseError> {
    TransactionInfo::withdraw_krw(amount, two_factor_type).await
}

/// 출금 허용 주소 리스트 조회
///
/// # Example
/// ```rust
/// let withdraw_addresses = api_withdraw::get_withdraw_address_list().await;
/// ```
/// # Response
/// ```json
/// [
///     {
///         "currency": "BTC",
///         "net_type": "BTC",
///         "network_name": "Bitcoin",
///         "withdraw_address": "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa",
///         "secondary_address": null
///     }
/// ]
/// ```
/// | field                  | description                   | type         |
/// |:-----------------------|:------------------------------|:-------------|
/// | currency | 출금 화폐 | String |
/// | net_type | 출금 네트워크 타입 | String |
/// | network_name | 출금 네트워크 이름 | String |
/// | withdraw_address | 출금 주소 | String |
/// | secondary_address | 2차 출금 주소 (필요한 디지털 자산에 한해서) | String |
pub async fn get_withdraw_address_list() -> Result<Vec<WithdrawCoinAddress>, ResponseError> {
    WithdrawCoinAddress::get_withdraw_address_list().await
}
