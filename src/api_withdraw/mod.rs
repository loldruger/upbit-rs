mod inquiry_withdraw_list;
mod inquiry_withdraw;

use crate::constant::OrderBy;
use super::response::{WithdrawInfo, ResponseError};

/// list of withdraw state
pub enum WithdrawState {
    Waiting,
    Processing,
    Done,
    Failed,
    Canceled,
    Rejected
}

impl ToString for WithdrawState {
    fn to_string(&self) -> String {
        match self {
            WithdrawState::Waiting => "waiting".to_owned(),
            WithdrawState::Processing => "processing".to_owned(),
            WithdrawState::Done => "done".to_owned(),
            WithdrawState::Failed => "failed".to_owned(),
            WithdrawState::Canceled => "canceled".to_owned(),
            WithdrawState::Rejected => "rejected".to_owned(),
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
/// let list_withdraw_info = api_withdraw::list_withdraw_info("KRW", WithdrawState::Done, None, None, 10, 0, OrderBy::Asc).await;
/// 
/// // it returns withdraw list of currency "BTC", state "done", txid "98c15999..." ordered by desc
/// let list_withdraw_info = api_withdraw::list_withdraw_info(
///     "BTC",
///     "done",
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
/// >> `WithdrawState::Waiting` 대기중<br>
/// >> `WithdrawState::Processing` 진행중<br>
/// >> `WithdrawState::Done` 완료<br>
/// >> `WithdrawState::Failed` 실패<br>
/// >> `WithdrawState::Canceled` 취소됨<br>
/// >> `WithdrawState::Rejected` 거절됨<br>
/// 
/// > `uuids` array of uuid<br>
/// > `txids` array of txid<br>
/// > `limit` pagination limit, max `100`<br>
/// > `page` pagination <br>
/// > `order_by` 
/// >> `OrderBy::Asc` 오름차순<br>
/// >> `OrderBy::Desc` 내림차순<br>
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
///  
///  * done_at field could be null depending on state
pub async fn list_withdraw_info(
    currency: &str,
    state: WithdrawState,
    uuids: Option<&[&str]>,
    txids: Option<&[&str]>,
    limit: u32,
    page: u32,
    order_by: OrderBy
) -> Result<Vec<WithdrawInfo>, ResponseError> {
    WithdrawInfo::inquiry_withdraw_list(currency, state, uuids, txids, limit, page, order_by).await
}

pub async fn get_withdraw_info(currency: Option<&str>, uuid: Option<&str>, txid: Option<&str>) -> Result<WithdrawInfo, ResponseError> {
    WithdrawInfo::get_withdraw_info(currency, uuid, txid).await
}
