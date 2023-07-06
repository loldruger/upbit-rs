use serde::Deserialize;

use super::{AccountsInfoSource, AccountsInfo};

#[derive(Debug)]
pub struct WithdrawInfo {
    pub r#type: String,
    pub uuid: String,
    pub currency: String,
    pub net_type: Option<String>,
    pub txid: String,
    pub state: String,
    pub created_at: chrono::NaiveDateTime,
    pub done_at: Option<chrono::NaiveDateTime>,
    pub amount: f64,
    pub fee: f64,
    pub transaction_type: String,
}

#[derive(Deserialize)]
pub struct WithdrawInfoSource {
    r#type: String,
    uuid: String,
    currency: String,
    net_type: Option<String>,
    txid: String,
    state: String,
    created_at: String,
    done_at: String,
    amount: String,
    fee: String,
    transaction_type: String
}

impl WithdrawInfoSource {
    pub fn r#type(&self) -> String { self.r#type.clone() }
    pub fn uuid(&self) -> String { self.uuid.clone() }
    pub fn currency(&self) -> String {self.currency.clone()}
    pub fn net_type(&self) -> Option<String> {self.net_type.clone()}
    pub fn txid(&self) -> String { self.txid.clone() }
    pub fn state(&self) -> String { self.state.clone() }
    pub fn created_at(&self) -> chrono::NaiveDateTime {chrono::NaiveDateTime::parse_from_str(&self.created_at, "%Y-%m-%dT%H:%M:%S%z").unwrap()}
    pub fn done_at(&self) -> Option<chrono::NaiveDateTime> {chrono::NaiveDateTime::parse_from_str(&self.done_at, "%Y-%m-%dT%H:%M:%S%z").ok()}
    pub fn amount(&self) -> f64 { self.amount.parse().unwrap() }
    pub fn fee(&self) -> f64 { self.fee.parse().unwrap() }
    pub fn transaction_type(&self) -> String { self.transaction_type.clone() }
}

pub struct WithdrawMemberLevel {
    security_level: i32,
    fee_level: i32,
    email_verified: bool,
    identity_auth_verified: bool,
    bank_account_verified: bool,
    kakao_pay_auth_verified: bool,
    locked: bool,
    wallet_locked: bool,
}

pub struct WithdrawCurrency {
    code: String,
    withdraw_fee: f64,
    is_coin: bool,
    wallet_state: String,
    wallet_support: Vec<String>
}

pub struct WithdrawCurrencySource {
    code: String,
    withdraw_fee: String,
    is_coin: bool,
    wallet_state: String,
    wallet_support: Vec<String>
}

pub struct WithdrawLimit {
    currency: String,
    minimum: Option<f64>,
    onetime: Option<f64>,
    daily: f64,
    remaining_daily: f64,
    remaining_daily_krw: f64,
    fixed: Option<i32>,
    can_withdraw: bool
}

pub struct WithdrawLimitSource {
    currency: String,
    minimum: Option<String>,
    onetime: Option<String>,
    daily: String,
    remaining_daily: String,
    remaining_daily_krw: String,
    fixed: Option<i32>,
    can_withdraw: bool
}

pub struct WithdrawChance {
    member_level: WithdrawMemberLevel,
    currency: WithdrawCurrency,
    account: AccountsInfo,
    withdraw_limit: WithdrawLimit
}

pub struct WithdrawChanceSource {
    member_level: WithdrawMemberLevel,
    currency: WithdrawCurrencySource,
    account: AccountsInfoSource,
    withdraw_limit: WithdrawLimitSource
}