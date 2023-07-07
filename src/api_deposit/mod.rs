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