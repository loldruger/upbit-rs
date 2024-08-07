use std::fmt::Display;

use crate::{
    constant::{OrderBy, TwoFactorType},
    response::{CoinAddressGen, CoinAddressResponse, ResponseError, TransactionInfo},
};

mod coin_address_generation;
mod coin_address_info;
mod coin_address_info_list;
mod deposit_info;
mod deposit_info_list;
mod deposit_krw;

/// List of kind of Deposit state
#[derive(Debug)]
pub enum DepositState {
    /// ???
    Done,
    /// 입금 진행중
    Processing,
    /// 완료
    Accepted,
    /// 취소됨
    Canceled,
    /// 거절됨
    Rejected,
    /// 트래블룰 추가 인증 대기중
    TravelRuleSuspected,
    /// 반환절차 진행중
    Refunding,
    /// 반환됨
    Refunded,
    /// ???
    Failed,
}

impl Display for DepositState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Done => write!(f, "DONE"),
            Self::Processing => write!(f, "PROCESSING"),
            Self::Accepted => write!(f, "ACCEPTED"),
            Self::Canceled => write!(f, "CANCELLED"), // this typo is intentional
            Self::Rejected => write!(f, "REJECTED"),
            Self::TravelRuleSuspected => write!(f, "TRAVEL_RULE_SUSPECTED"),
            Self::Refunding => write!(f, "REFUNDING"),
            Self::Refunded => write!(f, "REFUNDED"),
            Self::Failed => write!(f, "FAILED"),
        }
    }
}

impl From<&str> for DepositState {
    fn from(value: &str) -> Self {
        match value {
            "DONE" => Self::Done,
            "PROCESSING" => Self::Processing,
            "ACCEPTED" => Self::Accepted,
            "CANCELLED" => Self::Canceled,
            "REJECTED" => Self::Rejected,
            "TRAVEL_RULE_SUSPECTED" => Self::TravelRuleSuspected,
            "REFUNDING" => Self::Refunding,
            "REFUNDED" => Self::Refunded,
            "FAILED" => Self::Failed,
            a @ _ => panic!("Unexpected value: {}", a),
        }
    }
}

/// 입금 기록을 조회한다. (inquiry the records of deposits.)
///
/// # Example
/// ```rust
/// use constant::OrderBy;
/// use api_deposit::DepositState;
///
/// // it returns deposit list of currency "KRW", state "accepted" ordered by asc
/// let get_deposit_info_list = api_deposit::get_deposit_info_list("KRW", DepositState::Accepted, None, None, 10, 0, OrderBy::Asc).await;
///
/// // it returns deposit list of currency "BTC", state "accepted", txid "98c15999..." ordered by desc
/// let get_deposit_info_list = api_deposit::get_deposit_info_list(
///     "BTC",
///     "ACCEPTED",
///     None,
///     Some(&[
///         "98c15999f0bdc4ae0e8a-ed35868bb0c204fe6ec29e4058a3451e-88636d1040f4baddf943274ce37cf9cc",
///         ...
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
///  >> *  `DepositState::Processing` 입금 진행중<br>
///  >> *  `DepositState::Accepted` 완료<br>
///  >> *  `DepositState::Canceled` 취소됨<br>
///  >> *  `DepositState::Rejected` 거절됨<br>
///  >> *  `DepositState::TravelRuleSuspected` 트래블룰 추가 인증 대기중<br>
///  >> *  `DepositState::Refunding` 반환절차 진행중<br>
///  >> *  `DepositState::Refunded` 반환됨<br>
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
///     "type": "deposit",
///     "uuid": "94332e99-3a87-4a35-ad98-28b0c969f830",
///     "currency": "KRW",
///     "txid": "9e37c537-6849-4c8b-a134-57313f5dfc5a",
///     "state": "ACCEPTED",
///     "created_at": "2017-12-08T15:38:02+09:00",
///     "done_at": "2017-12-08T15:38:02+09:00",
///     "amount": "100000.0",
///     "fee": "0.0",
///     "transaction_type": "default"
///   }
///   #....
/// ]
/// ```
///
/// | field                  | description                   | type         |
/// |:-----------------------|:------------------------------|:-------------|
/// | type | 입출금 종류 | String
/// | uuid | 입금의 고유 아이디 | String
/// | currency | 화폐를 의미하는 영문 대문자 코드 | String
/// | net_type | 입금 네트워크 | String
/// | txid | 입금의 트랜잭션 아이디 | String
/// | state | 입금 상태<br> - PROCESSING  : 입금 진행중 <br> - ACCEPTED : 완료 <br> - CANCELLED : 취소됨<br> - REJECTED : 거절됨 <br> - TRAVEL_RULE_SUSPECTED : 트래블룰 추가 인증 대기중<br> - REFUNDING : 반환절차 진행중<br> - REFUNDED : 반환됨 | String
/// | created_at | 입금 생성 시간 | DateString
/// | done_at | 입금 완료 시간 | DateString
/// | amount | 입금 금액/수량 | NumberString
/// | fee | 입금 수수료 | NumberString
/// | transaction_type | 입금 유형<br> default : 일반입금<br>internal : 바로입금 | String
pub async fn get_deposit_info_list(
    currency: &str,
    state: DepositState,
    uuids: Option<&[&str]>,
    txids: Option<&[&str]>,
    limit: u32,
    page: u32,
    order_by: OrderBy,
) -> Result<Vec<TransactionInfo>, ResponseError> {
    TransactionInfo::get_deposit_list(currency, state, uuids, txids, limit, page, order_by).await
}

/// 개별 입금 조회.
///
/// # Example
/// ```rust
/// let deposit_result = api_deposit::get_deposit_info(Some("KRW"), None, None).await;
/// let deposit_result = api_deposit::get_deposit_info(None, Some("9f432943-54e0-40b7-825f-b6fec8b42b79"), None).await;
/// ```
/// - parameters
/// > `currency` ex) KRW, BTC, ETH etc. <br>
/// > `uuid` uuid<br>
/// > `txid` txid<br>
/// # Response
/// ```json
/// [
///   {
///     "type": "deposit",
///     "uuid": "94332e99-3a87-4a35-ad98-28b0c969f830",
///     "currency": "KRW",
///     "txid": "9e37c537-6849-4c8b-a134-57313f5dfc5a",
///     "state": "ACCEPTED",
///     "created_at": "2017-12-08T15:38:02+09:00",
///     "done_at": "2017-12-08T15:38:02+09:00",
///     "amount": "100000.0",
///     "fee": "0.0",
///     "transaction_type": "default"
///   }
///   #....
/// ]
/// ```
///
/// | field                  | description                   | type         |
/// |:-----------------------|:------------------------------|:-------------|
/// | type | 입출금 종류 | String
/// | uuid | 입금의 고유 아이디 | String
/// | currency | 화폐를 의미하는 영문 대문자 코드 | String
/// | net_type | 입금 네트워크 | String
/// | txid | 입금의 트랜잭션 아이디 | String
/// | state | 입금 상태<br> - WAITING : 대기중<br> - PROCESSING : 진행중<br> - DONE : 완료<br> - FAILED : 실패<br> - CANCELLED : 취소됨<br> - REJECTED : 거절됨 | String
/// | created_at | 입금 생성 시간 | DateString
/// | done_at | 입금 완료 시간 | DateString
/// | amount | 입금 금액/수량 | NumberString
/// | fee | 입금 수수료 | NumberString
/// | transaction_type | 입금 유형<br> default : 일반입금<br>internal : 바로입금 | String
pub async fn get_deposit_info(
    currency: Option<&str>,
    uuid: Option<&str>,
    txid: Option<&str>,
) -> Result<TransactionInfo, ResponseError> {
    TransactionInfo::get_deposit_info(currency, uuid, txid).await
}

/// 원화를 입금한다.
///
/// # Example
/// ```rust
/// let deposit_result = api_deposit::deposit_krw(10000.0, api_deposit::TwoFactorType::KakaoPay).await;
/// ```
/// - parameters
/// > `amount` amount of deposit <br>
/// > `two_factor_type`
/// >> * `TwoFactorType::KakaoPay` Two factor identification via kakao <br>
/// >> * `TwoFactorType::Naver` Two factor identification via naver <br>
/// # Response
/// ```json
/// {
///     "type": "deposit",
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
/// | type | 입출금 종류 | String |
/// | uuid | 입금의 고유 아이디 | String |
/// | currency | 화폐를 의미하는 영문 대문자 코드 | String |
/// | net_type | 입금 네트워크 | String |
/// | txid | 입금의 트랜잭션 아이디 | String |
/// | state | 입금 상태 | String |
/// | created_at | 입금 생성 시간 | DateString |
/// | done_at | 입금 완료 시간 | DateString |
/// | amount | 입금 금액/수량 | NumberString |
/// | fee | 입금 수수료 | NumberString |
/// | transaction_type | 입금 유형 | String |
pub async fn deposit_krw(
    amount: f64,
    two_factor_type: TwoFactorType,
) -> Result<TransactionInfo, ResponseError> {
    TransactionInfo::deposit_krw(amount, two_factor_type).await
}

/// 개별 입금 주소 조회
///
/// # Example
/// ```
/// let coin_address_info = api_deposit::get_coin_address_info("ETH", "ETH").await;
/// ```
/// - parameters
/// > `currency` ex) BTC, ETH etc. <br>
/// > `net_type` ex) BTC, ETH etc.
/// ```json
/// {
///    "currency": "ETH",
///    "net_type": "ETH",
///    "deposit_address": "0xe13ca9a87a5ab313ebf59f984e7e42690409120d",
///    "secondary_address": null
/// }
/// ```
/// | field                  | description                   | type         |
/// |:-----------------------|:------------------------------|:-------------|
/// | currency | 화폐를 의미하는 영문 대문자 코드 | String |
/// | net_type | 입금 네트워크 | String |
/// | deposit_address | 입금 주소 | String |
/// | secondary_address | 2차 입금 주소 | String |
pub async fn get_coin_address_info(
    currency: &str,
    net_type: &str,
) -> Result<CoinAddressResponse, ResponseError> {
    CoinAddressResponse::get_coin_address_info(currency, net_type).await
}

/// 전체 입금 주소 조회
///
/// # Example
/// ```
/// let coin_address_info_list = api_deposit::get_coin_address_info().await;
/// ```
/// # Response
/// ```json
/// [
///     {
///        "currency": "ETH",
///        "net_type": "ETH",
///        "deposit_address": "0xe13ca9a87a5ab313ebf59f984e7e42690409120d",
///        "secondary_address": null
///     },
///     ...
/// ]
/// ```
/// | field                  | description                   | type         |
/// |:-----------------------|:------------------------------|:-------------|
/// | currency | 화폐를 의미하는 영문 대문자 코드 | String |
/// | net_type | 입금 네트워크 | String |
/// | deposit_address | 입금 주소 | String |
/// | secondary_address | 2차 입금 주소 | String |
pub async fn get_coin_address_info_list() -> Result<Vec<CoinAddressResponse>, ResponseError> {
    CoinAddressResponse::get_coin_address_info_list().await
}

/// # Currently not working
pub async fn generate_deposit_address(
    currency: &str,
    net_type: &str,
) -> Result<CoinAddressGen, ResponseError> {
    CoinAddressGen::generate_deposit_address(currency, net_type).await
}
