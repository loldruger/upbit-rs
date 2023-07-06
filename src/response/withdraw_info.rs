use serde::Deserialize;

use super::{AccountsInfoSource, AccountsInfo};

/// Deserialized derived WithdrawInfo data
#[derive(Debug)]
pub struct WithdrawInfoDerived {
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
    pub krw_amount: f64,
    pub transaction_type: String,
}

/// Raw derived withdraw info from serialized data
#[derive(Deserialize)]
pub struct WithdrawInfoDerivedSource {
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
    krw_amount: String,
    transaction_type: String
}

impl WithdrawInfoDerivedSource {
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
    pub fn krw_amount(&self) -> f64 { self.krw_amount.parse().unwrap() }
    pub fn transaction_type(&self) -> String { self.transaction_type.clone() }
}

/// Deserialized WithdrawInfo data
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

/// Raw withdraw info from serialized data
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

/// Raw MemberLevel of WithdrawChanceSource from serialized data
#[derive(Deserialize, Debug)]
pub struct MemberLevel {
    pub security_level: i32,
    pub fee_level: i32,
    pub email_verified: bool,
    pub identity_auth_verified: bool,
    pub bank_account_verified: bool,
    pub kakao_pay_auth_verified: bool,
    pub locked: bool,
    pub wallet_locked: bool,
}

/// Deserialized WithdrawCurrency of WithdrawChance data
#[derive(Debug)]
pub struct WithdrawCurrency {
    pub code: String,
    pub withdraw_fee: f64,
    pub is_coin: bool,
    pub wallet_state: String,
    pub wallet_support: Vec<String>
}

/// Raw withdraw currency from serialized data
#[derive(Deserialize)]
pub struct WithdrawCurrencySource {
    code: String,
    withdraw_fee: String,
    is_coin: bool,
    wallet_state: String,
    wallet_support: Vec<String>
}

impl WithdrawCurrencySource {
    pub fn code(&self) -> String { self.code.clone() }
    pub fn withdraw_fee(&self) -> f64 { self.withdraw_fee.parse().unwrap() }
    pub fn is_coin(&self) -> bool { self.is_coin }
    pub fn wallet_state(&self) -> String { self.wallet_state.clone() }
    pub fn wallet_support(&self) -> Vec<String> { self.wallet_support.clone() }
}

/// Deserialized WithdrawLimit of WithdrawChanceSource data
#[derive(Debug)]
pub struct WithdrawLimit {
    pub currency: String,
    pub minimum: Option<f64>,
    pub onetime: Option<f64>,
    pub daily: f64,
    pub remaining_daily: f64,
    pub remaining_daily_krw: f64,
    pub fixed: Option<i32>,
    pub can_withdraw: bool
}

/// Raw withdraw limit from serialized data
#[derive(Deserialize)]
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

impl WithdrawLimitSource {
    pub fn currency(&self) -> String { self.currency.clone() }
    pub fn minimum(&self) -> Option<f64> { self.minimum.clone().map(|x| x.parse::<f64>().unwrap()) }
    pub fn onetime(&self) -> Option<f64> { self.onetime.clone().map(|x| x.parse::<f64>().unwrap()) }
    pub fn daily(&self) -> f64 { self.daily.parse().unwrap() }
    pub fn remaining_daily(&self) -> f64 { self.remaining_daily.parse().unwrap() }
    pub fn remaining_daily_krw(&self) -> f64 { self.remaining_daily_krw.parse().unwrap() }
    pub fn fixed(&self) -> Option<i32> { self.fixed }
    pub fn can_withdraw(&self) -> bool { self.can_withdraw }
}

/// Deserialized WithdrawChance of WithdrawChanceSource data
#[derive(Debug)]
pub struct WithdrawChance {
    pub member_level: MemberLevel,
    pub currency: WithdrawCurrency,
    pub account: AccountsInfo,
    pub withdraw_limit: WithdrawLimit
}

/// Raw withdraw chance info from serialized data
#[derive(Deserialize)]
pub struct WithdrawChanceSource {
    pub member_level: MemberLevel,
    pub currency: WithdrawCurrencySource,
    pub account: AccountsInfoSource,
    pub withdraw_limit: WithdrawLimitSource
}

