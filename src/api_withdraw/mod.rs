mod inquiry_withdraw;
use inquiry_withdraw::*;

use super::response::{WithdrawInfo, ResponseError};

/// 마켓별 주문 가능 정보를 확인한다.
/// ```rust
/// let list_withdraw_info = api::list_withdraw_info().await;
/// ```
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
pub async fn list_withdraw_info() -> Result<Vec<WithdrawInfo>, ResponseError> {
    WithdrawInfo::inquiry_withdraw_list().await
}