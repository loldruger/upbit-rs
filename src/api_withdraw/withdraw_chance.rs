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
        ResponseError
    },
};

impl RequestWithQuery for WithdrawChance {}
impl WithdrawChance {
    pub async fn get_withdraw_chance(currency: &str, net_type: Option<&str>) -> Result<Self, ResponseError> {
        let res = Self::request(currency, net_type).await?;
        let res_serialized = res.text().await.map_err(crate::response::response_error_from_reqwest)?;
        
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

        if let Some(net_type) = net_type {
            url.query_pairs_mut().append_pair("net_type", net_type);
        }

        reqwest::Client::new()
            .get(url)
            .header(ACCEPT, "application/json")
            .header(AUTHORIZATION, &token_string)
            .send()
            .await
            .map_err(crate::response::response_error_from_reqwest)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use serde_json::Value;

    use super::*;

    #[tokio::test]
    async fn test_get_withdraw_chance() {
        crate::set_access_key(&std::env::var("TEST_ACCESS_KEY").expect("TEST_ACCESS_KEY not set"));
        crate::set_secret_key(&std::env::var("TEST_SECRET_KEY").expect("TEST_SECRET_KEY not set"));
    
        let res = WithdrawChance::request("KRW-ETH", None).await.unwrap();
        let res_serialized = res
            .text()
            .await
            .map_err(crate::response::response_error_from_reqwest)
            .unwrap();

        if res_serialized.contains("error") {
            assert!(false, "Error response: {res_serialized}");
        }
        
        let json = serde_json::from_str::<Value>(&res_serialized).unwrap();
        let expected_structure = serde_json::json!(
            {
                "bid_fee": "",
                "ask_fee": "",
                "market": {
                    "id": "",
                    "name": "",
                    "order_sides": [],
                    "order_types": [],
                    "bid": {
                        "currency": "",
                        "min_total": ""
                    },
                    "ask": {
                        "currency": "",
                        "min_total": ""
                    },
                    "max_total": "",
                    "state": "",
                    "price_unit": "",
                    "quantity_unit": "",
                    "price_precision": "",
                    "quantity_precision": "",
                    "min_total": "",
                    "state_reason": "",
                    "data": {
                        "message": "",
                        "code": ""
                    },
                    "maker_bid_fee": "",
                    "maker_ask_fee": ""
                }
            }
        );
    
        let expected_structure = expected_structure
            .as_object()
            .unwrap()
            .iter()
            .map(|(k, v)| (k.as_str(), v.clone()))
            .collect::<HashMap<&str, Value>>();
    
        let (missing_keys, extra_keys) = compare_keys(&json, &expected_structure, "");
    
        if !missing_keys.is_empty() {
            println!("[test_get_withdraw_chance] Missing keys: {:?}", missing_keys);
            assert!(false);
        } else {
            println!("[test_get_withdraw_chance] No keys are missing");
            assert!(true);
        }
    
        if !extra_keys.is_empty() {
            println!("[test_get_withdraw_chance] Extra keys: {:?}", extra_keys);
            assert!(false);
        } else {
            println!("[test_get_withdraw_chance] No extra keys found.");
            assert!(true);
        }
    }

    fn compare_keys(json: &Value, expected: &HashMap<&str, Value>, path: &str) -> (Vec<String>, Vec<String>) {
        let mut missing_keys = Vec::new();
        let mut extra_keys = Vec::new();
    
        if let Value::Object(map) = json {
            let json_keys: HashSet<&str> = map.keys().map(|k| k.as_str()).collect();
            let expected_keys: HashSet<&str> = expected.keys().cloned().collect();
    
            for key in expected_keys.difference(&json_keys) {
                missing_keys.push(format!("{}{}", path, key));
            }
    
            for key in json_keys.difference(&expected_keys) {
                extra_keys.push(format!("{}{}", path, key));
            }
    
            for key in expected_keys.intersection(&json_keys) {
                if let Some(expected_value) = expected.get(*key) {
                    let new_path = format!("{}{}.", path, key);
                    if let Value::Object(_) = expected_value {
                        let expected_map = expected_value.as_object().unwrap().iter().map(|(k, v)| (k.as_str(), v.clone())).collect::<HashMap<&str, Value>>();
                        let (mut missing, mut extra) = compare_keys(&map[*key], &expected_map, &new_path);
                        missing_keys.append(&mut missing);
                        extra_keys.append(&mut extra);
                    }
                }
            }
        }
    
        (missing_keys, extra_keys)
    }
}