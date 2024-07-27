use reqwest::{Response, Url};
use reqwest::header::{ACCEPT, AUTHORIZATION};

use crate::request::RequestWithQuery;

use super::{
    super::constant::{URL_WITHDRAWS_CHANCE, URL_SERVER},
    super::response::{
        AccountsInfo,
        MemberLevel,
        WithdrawLimit,
        WithdrawChance,
        WithdrawChanceSource,
        WithdrawCurrency,
        ResponseError,
        ResponseErrorBody,
        ResponseErrorState
    },
};

impl RequestWithQuery for WithdrawChance {}
impl WithdrawChance {
    pub async fn get_withdraw_chance(currency: &str, net_type: Option<&str>) -> Result<Self, ResponseError> {
        let res = Self::request(currency, net_type).await?;
        let res_serialized = res.text().await.unwrap();
        
        if res_serialized.contains("error") {
            return Err(serde_json::from_str(&res_serialized)
                .map(crate::response::response_error)                
                .ok()
                .unwrap()
            )
        }

        serde_json::from_str(&res_serialized)
            .map(|x: WithdrawChanceSource| {
                Self {
                    member_level: MemberLevel {
                        security_level: x.member_level.security_level,
                        fee_level: x.member_level.fee_level,
                        email_verified: x.member_level.email_verified,
                        identity_auth_verified: x.member_level.identity_auth_verified,
                        bank_account_verified: x.member_level.bank_account_verified,
                        kakao_pay_auth_verified: x.member_level.kakao_pay_auth_verified,
                        locked: x.member_level.locked,
                        wallet_locked: x.member_level.wallet_locked,
                    },
                    currency: WithdrawCurrency {
                        code: x.currency.code(),
                        withdraw_fee: x.currency.withdraw_fee(),
                        is_coin: x.currency.is_coin(),
                        wallet_state: x.currency.wallet_state(),
                        wallet_support: x.currency.wallet_support(),
                    },
                    account: AccountsInfo {
                        currency: x.account.currency(),
                        balance: x.account.balance(),
                        locked: x.account.locked(),
                        avg_buy_price: x.account.avg_buy_price(),
                        avg_buy_price_modified: x.account.avg_buy_price_modified(),
                        unit_currency: x.account.unit_currency(),
                    },
                    withdraw_limit: WithdrawLimit {
                        currency: x.withdraw_limit.currency(),
                        minimum: x.withdraw_limit.minimum(),
                        onetime: x.withdraw_limit.onetime(),
                        daily: x.withdraw_limit.daily(),
                        remaining_daily: x.withdraw_limit.remaining_daily(),
                        remaining_daily_krw: x.withdraw_limit.remaining_daily_krw(),
                        fixed: x.withdraw_limit.fixed(),
                        can_withdraw: x.withdraw_limit.can_withdraw(),
                    },
                }
            })
            .map_err(crate::response::response_error_from_json)
    }

    async fn request(currency: &str, net_type: Option<&str>) -> Result<Response, ResponseError> {
        let mut url = Url::parse(&format!("{URL_SERVER}{URL_WITHDRAWS_CHANCE}?currency={currency}")).unwrap();
        let token_string = Self::set_token_with_query(url.as_str())?;

        if net_type.is_some() {
            url.query_pairs_mut().append_pair("net_type", net_type.unwrap());
        }

        reqwest::Client::new()
            .get(url)
            .header(ACCEPT, "application/json")
            .header(AUTHORIZATION, &token_string)
            .send()
            .await
            .map_err(|x| {
                ResponseError {
                    state: ResponseErrorState::InternalReqwestError,
                    error: ResponseErrorBody {
                        name: "internal_reqwest_error".to_owned(),
                        message: x.to_string()
                    }
                }
            })
    }
}
