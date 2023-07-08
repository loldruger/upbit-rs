use crate::{response::{WithdrawInfo, ResponseError}, constant::OrderBy};

mod deposit_info;
mod deposit_info_list;
mod deposit_krw_requestion;
mod coin_address_generation;
mod coin_address_inquiry;
mod coin_addresses_inquiry;

/// List of kind of Deposit state 
pub enum DepositState {
    Processing,
    // working
    Accepted,
    Canceled,
    // working
    Rejected,
    TravelRuleSuspected,
    Refunding,
    Refunded
}

impl ToString for DepositState {
    fn to_string(&self) -> String {
        match self {
            DepositState::Processing => "processing".to_owned(),
            DepositState::Accepted => "accepted".to_owned(),
            DepositState::Canceled => "cancelled".to_owned(), // this typo is intentional
            DepositState::Rejected => "rejected".to_owned(),
            DepositState::TravelRuleSuspected => "travel_rule_suspected".to_owned(),
            DepositState::Refunding => "refunding".to_owned(),
            DepositState::Refunded => "refunded".to_owned(),
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
/// let list_deposit_info = api_deposit::list_deposit_info("KRW", DepositState::Accepted, None, None, 10, 0, OrderBy::Asc).await;
/// 
/// // it returns deposit list of currency "BTC", state "accepted", txid "98c15999..." ordered by desc
/// let list_deposit_info = api_deposit::list_deposit_info(
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
/// | uuid | 출금의 고유 아이디 | String
/// | currency | 화폐를 의미하는 영문 대문자 코드 | String
/// | net_type | 출금 네트워크 | String
/// | txid | 출금의 트랜잭션 아이디 | String
/// | state | 출금 상태<br> - PROCESSING  : 입금 진행중 <br> - ACCEPTED : 완료 <br> - CANCELLED : 취소됨<br> - REJECTED : 거절됨 <br> - TRAVEL_RULE_SUSPECTED : 트래블룰 추가 인증 대기중<br> - REFUNDING : 반환절차 진행중<br> - REFUNDED : 반환됨 | String
/// | created_at | 출금 생성 시간 | DateString
/// | done_at | 출금 완료 시간 | DateString
/// | amount | 출금 금액/수량 | NumberString
/// | fee | 출금 수수료 | NumberString
/// | transaction_type | 출금 유형<br> default : 일반출금<br>internal : 바로출금 | String
pub async fn list_deposit_info(
    currency: &str,
    state: DepositState,
    uuids: Option<&[&str]>,
    txids: Option<&[&str]>,
    limit: u32,
    page: u32,
    order_by: OrderBy
) -> Result<Vec<WithdrawInfo>, ResponseError> {
    WithdrawInfo::inquiry_deposit_list(currency, state, uuids, txids, limit, page, order_by).await
}