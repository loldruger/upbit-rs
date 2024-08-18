use serde::{Deserialize, Serialize};

use crate::{
    api_deposit::DepositState,
    constant::{TransactionType, TransferType},
    request::{Request, RequestWithQuery},
};

use super::{AccountsInfo, AccountsInfoSource};

/// Deserialized derived TransactionInfoDerived data
#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionInfoDerived {
    pub r#type: TransferType,
    pub uuid: String,
    pub currency: String,
    pub net_type: Option<String>,
    pub txid: String,
    pub state: DepositState,

    #[cfg(feature = "chrono")]
    pub created_at: chrono::NaiveDateTime,
    #[cfg(not(any(feature = "chrono")))]
    pub created_at: String,

    #[cfg(feature = "chrono")]
    pub done_at: Option<chrono::NaiveDateTime>,
    #[cfg(not(any(feature = "chrono")))]
    pub done_at: Option<String>,

    pub amount: f64,
    pub fee: f64,
    // pub krw_amount: f64,
    pub transaction_type: TransactionType,
}

/// Raw derived withdraw info from serialized data
#[derive(Deserialize)]
pub struct TransactionInfoDerivedSource {
    r#type: String,
    uuid: String,
    currency: String,
    net_type: Option<String>,
    txid: String,
    state: String,
    created_at: String,
    done_at: Option<String>,
    amount: String,
    fee: String,
    // krw_amount: String,
    transaction_type: String,
}

impl TransactionInfoDerivedSource {
    /// Convert [String] type value into [TransactionType]
    pub fn r#type(&self) -> TransferType {
        self.r#type.as_str().into()
    }
    /// Get uuid
    pub fn uuid(&self) -> String {
        self.uuid.clone()
    }
    /// Get currency
    pub fn currency(&self) -> String {
        self.currency.clone()
    }
    /// Get net_type
    pub fn net_type(&self) -> Option<String> {
        self.net_type.clone().or(None)
    }
    /// Get txid
    pub fn txid(&self) -> String {
        self.txid.clone()
    }
    /// Convert [String] state value into [DepositState]
    pub fn state(&self) -> DepositState {
        self.state.as_str().into()
    }

    #[cfg(not(any(feature = "chrono")))]
    /// Convert [String] created_at value into [chrono::NaiveDateTime]
    pub fn created_at(&self) -> String {
        self.created_at.clone()
    }

    #[cfg(feature = "chrono")]
    pub fn created_at(&self) -> chrono::NaiveDateTime {
        chrono::DateTime::parse_from_rfc3339(&self.created_at).map(|dt| dt.naive_local()).unwrap()
    }

    #[cfg(not(any(feature = "chrono")))]
    /// Convert [String] done_at value into [chrono::NaiveDateTime]
    pub fn done_at(&self) -> Option<String> {
        self.done_at.clone().or(None)
    }

    #[cfg(feature = "chrono")]
    pub fn done_at(&self) -> Option<chrono::NaiveDateTime> {
        chrono::NaiveDateTime::parse_from_str(
            &self.done_at.clone().or(None)?,
            "%Y-%m-%dT%H:%M:%S%z",
        )
        .ok()
    }
    
    /// Convert [String] amount value into [f64]
    pub fn amount(&self) -> f64 {
        self.amount.parse().unwrap()
    }
    /// Convert [String] fee value into [f64]
    pub fn fee(&self) -> f64 {
        self.fee.parse().unwrap()
    }
    /// Convert [String] krw_amount value into [f64]
    // pub fn krw_amount(&self) -> f64 { self.krw_amount.parse().unwrap() }
    /// Convert [String] transaction_type value into [WithdrawType]
    pub fn transaction_type(&self) -> TransactionType {
        self.transaction_type.as_str().into()
    }
}

/// Deserialized TransactionInfo data
#[derive(Debug)]
pub struct TransactionInfo {
    pub r#type: TransferType,
    pub uuid: String,
    pub currency: String,
    pub net_type: Option<String>,
    pub txid: String,
    pub state: DepositState,

    #[cfg(feature = "chrono")]
    pub created_at: chrono::NaiveDateTime,
    #[cfg(not(any(feature = "chrono")))]
    pub created_at: String,

    #[cfg(feature = "chrono")]
    pub done_at: Option<chrono::NaiveDateTime>,
    #[cfg(not(any(feature = "chrono")))]
    pub done_at: Option<String>,

    pub amount: f64,
    pub fee: f64,
    pub transaction_type: TransactionType,

    pub holder: Option<String>,
    pub bank: Option<String>,
    pub fiat_amount: Option<String>,
    pub memo: Option<String>,
    pub fiat_currency: Option<String>,
    pub confirmations: Option<String>,
    pub krw_amount: Option<String>,
    pub network_name: Option<String>,
    pub cancelable: Option<String>,
    pub blockchain_url: Option<String>,
    pub state_i18n: Option<String>,
    pub address: Option<String>,
}

impl RequestWithQuery for TransactionInfo {}

/// Raw withdraw info from serialized data
#[derive(Deserialize)]
pub struct TransactionInfoSource {
    r#type: String,
    uuid: String,
    currency: String,
    net_type: Option<String>,
    txid: String,
    state: String,
    created_at: String,
    done_at: Option<String>,
    amount: String,
    fee: String,
    transaction_type: String,

    holder: Option<String>,
    bank: Option<String>,
    fiat_amount: Option<String>,
    memo: Option<String>,
    fiat_currency: Option<String>,
    confirmations: Option<String>,
    krw_amount: Option<String>,
    network_name: Option<String>,
    cancelable: Option<String>,
    blockchain_url: Option<String>,
    state_i18n: Option<String>,
    address: Option<String>,
}

impl TransactionInfoSource {
    /// Convert [String] type value into [TransactionType]
    pub fn r#type(&self) -> TransferType {
        self.r#type.as_str().into()
    }
    /// Get uuid
    pub fn uuid(&self) -> String {
        self.uuid.clone()
    }
    /// Get currency
    pub fn currency(&self) -> String {
        self.currency.clone()
    }
    /// Get net_type
    pub fn net_type(&self) -> Option<String> {
        self.net_type.clone().or(None)
    }
    /// Get txid
    pub fn txid(&self) -> String {
        self.txid.clone()
    }
    
    /// Convert [String] state value into [DepositState]
    pub fn state(&self) -> DepositState {
        self.state.as_str().into()
    }

    #[cfg(not(any(feature = "chrono")))]
    /// Convert [String] created_at value into [chrono::NaiveDateTime]
    pub fn created_at(&self) -> String {
        self.created_at.clone()
    }

    #[cfg(feature = "chrono")]
    pub fn created_at(&self) -> chrono::NaiveDateTime {
        chrono::DateTime::parse_from_rfc3339(&self.created_at).map(|dt| dt.naive_local()).unwrap()
    }

    #[cfg(not(any(feature = "chrono")))]
    /// Convert [String] done_at value into [chrono::NaiveDateTime]
    pub fn done_at(&self) -> Option<String> {
        self.done_at.clone().or(None)
    }

    #[cfg(feature = "chrono")]
    pub fn done_at(&self) -> Option<chrono::NaiveDateTime> {
        chrono::NaiveDateTime::parse_from_str(
            &self.done_at.clone().or(None)?,
            "%Y-%m-%dT%H:%M:%S%z",
        )
        .ok()
    }

    /// Convert [String] amount value into [f64]
    pub fn amount(&self) -> f64 {
        self.amount.parse().unwrap()
    }
    /// Convert [String] fee value into [f64]
    pub fn fee(&self) -> f64 {
        self.fee.parse().unwrap()
    }
    /// Convert [String] transaction_type value into [WithdrawType]
    pub fn transaction_type(&self) -> TransactionType {
        self.transaction_type.as_str().into()
    }
    pub fn holder(&self) -> Option<String> {
        self.holder.clone()
    }
    pub fn bank(&self) -> Option<String> {
        self.bank.clone()
    }
    pub fn fiat_amount(&self) -> Option<String> {
        self.fiat_amount.clone()
    }
    pub fn memo(&self) -> Option<String> {
        self.memo.clone()
    }
    pub fn fiat_currency(&self) -> Option<String> {
        self.fiat_currency.clone()
    }
    pub fn confirmations(&self) -> Option<String> {
        self.confirmations.clone()
    }
    pub fn krw_amount(&self) -> Option<String> {
        self.krw_amount.clone()
    }
    pub fn network_name(&self) -> Option<String> {
        self.network_name.clone()
    }
    pub fn cancelable(&self) -> Option<String> {
        self.cancelable.clone()
    }
    pub fn blockchain_url(&self) -> Option<String> {
        self.blockchain_url.clone()
    }
    pub fn state_i18n(&self) -> Option<String> {
        self.state_i18n.clone()
    }
    pub fn address(&self) -> Option<String> {
        self.address.clone()
    }
}

/// Raw MemberLevel of [WithdrawChanceSource] from serialized data
#[derive(Deserialize, Debug)]
pub struct MemberLevel {
    pub security_level: i32,
    pub fee_level: i32,
    pub email_verified: bool,
    pub identity_auth_verified: bool,
    pub bank_account_verified: bool,
    pub two_factor_auth_verified: bool,
    // pub kakao_pay_auth_verified: bool,
    pub locked: bool,
    pub wallet_locked: bool,
}

/// Deserialized WithdrawCurrency of [WithdrawChance] data
#[derive(Debug)]
pub struct WithdrawCurrency {
    pub code: String,
    pub withdraw_fee: f64,
    pub is_coin: bool,
    pub wallet_state: String,
    pub wallet_support: Vec<String>,
}

/// Raw withdraw currency from serialized data
#[derive(Deserialize)]
pub struct WithdrawCurrencySource {
    code: String,
    withdraw_fee: String,
    is_coin: bool,
    wallet_state: String,
    wallet_support: Vec<String>,
}

impl WithdrawCurrencySource {
    pub fn code(&self) -> String {
        self.code.clone()
    }
    pub fn withdraw_fee(&self) -> f64 {
        self.withdraw_fee.parse().unwrap()
    }
    pub fn is_coin(&self) -> bool {
        self.is_coin
    }
    pub fn wallet_state(&self) -> String {
        self.wallet_state.clone()
    }
    pub fn wallet_support(&self) -> Vec<String> {
        self.wallet_support.clone()
    }
}

/// Deserialized WithdrawLimit of [WithdrawChanceSource] data
#[derive(Debug)]
pub struct WithdrawLimit {
    pub currency: String,
    pub minimum: Option<f64>,
    #[deprecated(since = "1.7.3", note = "Use remaining_daily_fiat instead")]
    pub onetime: Option<f64>,
    #[deprecated(since = "1.7.3", note = "Use remaining_daily_fiat instead")]
    pub daily: Option<f64>,
    #[deprecated(since = "1.7.3", note = "Use remaining_daily_fiat instead")]
    pub remaining_daily: f64,
    #[deprecated(since = "1.7.3", note = "Use remaining_daily_fiat instead")]
    pub remaining_daily_krw: f64,
    pub remaining_daily_fiat: f64,
    pub fixed: Option<i32>,
    pub can_withdraw: bool,
}

/// Raw withdraw limit from serialized data
#[derive(Deserialize)]
pub struct WithdrawLimitSource {
    currency: String,
    minimum: Option<String>,
    #[deprecated(since = "1.7.3", note = "Use remaining_daily_fiat instead")]
    onetime: Option<String>,
    #[deprecated(since = "1.7.3", note = "Use remaining_daily_fiat instead")]
    daily: Option<String>,
    #[deprecated(since = "1.7.3", note = "Use remaining_daily_fiat instead")]
    remaining_daily: String,
    #[deprecated(since = "1.7.3", note = "Use remaining_daily_fiat instead")]
    remaining_daily_krw: String,
    remaining_daily_fiat: String,
    fixed: Option<i32>,
    can_withdraw: bool,
}

impl WithdrawLimitSource {
    pub fn currency(&self) -> String {
        self.currency.clone()
    }
    pub fn minimum(&self) -> Option<f64> {
        self.minimum.clone().map(|x| x.parse::<f64>().unwrap())
    }
    #[allow(deprecated)]
    #[deprecated(since = "1.7.3", note = "Use remaining_daily_fiat instead")]
    pub fn onetime(&self) -> Option<f64> {
        self.onetime.clone().map(|x| x.parse::<f64>().unwrap())
    }
    #[allow(deprecated)]
    #[deprecated(since = "1.7.3", note = "Use remaining_daily_fiat instead")]
    pub fn daily(&self) -> Option<f64> {
        self.daily.as_ref().and_then(|x| x.parse::<f64>().ok())
    }
    #[allow(deprecated)]
    #[deprecated(since = "1.7.3", note = "Use remaining_daily_fiat instead")]
    pub fn remaining_daily(&self) -> f64 {
        self.remaining_daily.parse().unwrap()
    }
    #[allow(deprecated)]
    #[deprecated(since = "1.7.3", note = "Use remaining_daily_fiat instead")]
    pub fn remaining_daily_krw(&self) -> f64 {
        self.remaining_daily_krw.parse().unwrap()
    }
    pub fn remaining_daily_fiat(&self) -> f64 {
        self.remaining_daily_fiat.parse().unwrap()
    }
    pub fn fixed(&self) -> Option<i32> {
        self.fixed
    }
    pub fn can_withdraw(&self) -> bool {
        self.can_withdraw
    }
}

/// Deserialized WithdrawChance of [WithdrawChanceSource] data
#[derive(Debug)]
pub struct WithdrawChance {
    pub member_level: MemberLevel,
    pub currency: WithdrawCurrency,
    pub account: AccountsInfo,
    pub withdraw_limit: WithdrawLimit,
}

/// Raw withdraw chance info from serialized data
#[derive(Deserialize)]
pub struct WithdrawChanceSource {
    pub member_level: MemberLevel,
    pub currency: WithdrawCurrencySource,
    pub account: AccountsInfoSource,
    pub withdraw_limit: WithdrawLimitSource,
}

/// Raw withdraw chance info from serialized data
#[derive(Deserialize, Debug)]
pub struct WithdrawCoinAddress {
    pub currency: String,
    pub net_type: String,
    pub network_name: String,
    pub withdraw_address: String,
    pub secondary_address: Option<String>,
}

impl Request for WithdrawCoinAddress {}

/// Kind of response body of coin address Generator
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum CoinAddressGenResponse {
    First(CoinAddressGenFirstResponse),
    Second(CoinAddressGenSecondaryResponse),
}

/// Response body of coin address generator
#[derive(Deserialize, Debug)]
pub struct CoinAddressGen {
    pub response: CoinAddressGenResponse,
}

impl RequestWithQuery for CoinAddressGen {}

/// Raw CoinAddressGenFirstResponse from serialized data
///
/// Which is first response
#[derive(Deserialize, Debug)]
pub struct CoinAddressGenFirstResponse {
    pub success: bool,
    pub message: String,
}

/// Raw CoinAddressGenSecondResponse from serialized data
#[derive(Deserialize, Debug)]
pub struct CoinAddressGenSecondaryResponse {
    pub currency: String,
    pub net_type: Option<String>,
    pub deposit_address: String,
    pub secondary_address: Option<String>,
}

/// Response body of coin address info
#[derive(Deserialize, Debug)]
pub struct CoinAddressResponse {
    pub currency: String,
    pub net_type: String,
    pub deposit_address: Option<String>,
    pub secondary_address: Option<String>,
}

impl Request for CoinAddressResponse {}
impl RequestWithQuery for CoinAddressResponse {}
