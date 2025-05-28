use std::collections::{HashMap, HashSet};
use serde_json::Value;

/// JSON 응답과 예상 구조를 비교하여 누락되거나 추가된 키를 찾는 함수
pub fn compare_keys(
    json: &Value,
    expected: &HashMap<&str, Value>,
    path: &str,
) -> (Vec<String>, Vec<String>) {
    let mut missing_keys = Vec::new();
    let mut extra_keys = Vec::new();

    if let Value::Object(map) = json {
        let json_keys: HashSet<&str> = map.keys().map(|k| k.as_str()).collect();
        let expected_keys: HashSet<&str> = expected.keys().cloned().collect();

        // 누락된 키 찾기
        for key in expected_keys.difference(&json_keys) {
            missing_keys.push(format!("{}{}", path, key));
        }

        // 추가된 키 찾기
        for key in json_keys.difference(&expected_keys) {
            extra_keys.push(format!("{}{}", path, key));
        }

        // 중첩된 객체 재귀적으로 확인
        for key in expected_keys.intersection(&json_keys) {
            if let Some(expected_value) = expected.get(*key) {
                let new_path = format!("{}{}.", path, key);
                if let Value::Object(_) = expected_value {
                    let expected_map = expected_value
                        .as_object()
                        .unwrap()
                        .iter()
                        .map(|(k, v)| (k.as_str(), v.clone()))
                        .collect::<HashMap<&str, Value>>();
                    let (mut missing, mut extra) =
                        compare_keys(&map[*key], &expected_map, &new_path);
                    missing_keys.append(&mut missing);
                    extra_keys.append(&mut extra);
                }
            }
        }
    }

    (missing_keys, extra_keys)
}

/// 테스트에서 API 키 설정을 위한 헬퍼 함수
pub fn setup_test_keys() {
    // .env 파일에서 자동으로 로딩하도록 개선
    if let Err(e) = crate::init_for_test() {
        panic!("Failed to initialize test keys: {}. Please check your .env file.", e);
    }
}

/// 테스트 결과 검증을 위한 헬퍼 함수
pub fn assert_no_missing_or_extra_keys(
    missing_keys: &[String],
    extra_keys: &[String],
    test_name: &str,
) {
    if !missing_keys.is_empty() {
        eprintln!("[{}] Missing keys: {:?}", test_name, missing_keys);
        panic!("Missing keys found in {}", test_name);
    }

    if !extra_keys.is_empty() {
        eprintln!("[{}] Extra keys: {:?}", test_name, extra_keys);
        panic!("Extra keys found in {}", test_name);
    }

    println!("[{}] All key validation passed", test_name);
}

/// 에러 응답 체크를 위한 헬퍼 함수
pub fn check_error_response(response_text: &str, test_name: &str) {
    if response_text.contains("error") {
        panic!("Error response in {}: {}", test_name, response_text);
    }
}
