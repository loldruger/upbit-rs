/// Module for deposit APIs
pub mod api_deposit;
/// Module for exchange APIs
pub mod api_exchange;
/// Module for quotation APIs
pub mod api_quotation;
/// Module for withdrawal APIs
pub mod api_withdraw;
/// Set of constants
pub mod constant;
/// Set of concrete request bodies
pub mod request;
/// Set of concrete response bodies
pub mod response;

pub use request::{Request, RequestWithQuery};

use std::sync::Once;

static INIT: Once = Once::new();

/// .env 파일을 로드하는 함수 (한 번만 실행됨)
fn load_dotenv() {
    INIT.call_once(|| {
        if let Err(e) = dotenv::dotenv() {
            // .env 파일이 없거나 로드할 수 없는 경우 경고만 출력
            eprintln!("Warning: Could not load .env file: {e}");
        }
    });
}

/// Upbit API 클라이언트를 초기화합니다.
///
/// 이 함수는 다음을 수행합니다:
/// 1. .env 파일에서 환경변수를 로드합니다.
/// 2. UPBIT_ACCESS_KEY와 UPBIT_SECRET_KEY 환경변수를 확인합니다.
///
/// # Examples
///
/// ```rust
/// use upbit;
///
/// // .env 파일 또는 환경변수에서 자동으로 키를 로드
/// match upbit::init() {
///     Ok(()) => println!("Upbit API initialized successfully"),
///     Err(e) => eprintln!("Failed to initialize: {}", e),
/// }
/// ```
pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    load_dotenv();

    let access_key = std::env::var("UPBIT_ACCESS_KEY")
        .map_err(|_| "UPBIT_ACCESS_KEY environment variable not found. Please set it in .env file or environment.")?;

    let secret_key = std::env::var("UPBIT_SECRET_KEY")
        .map_err(|_| "UPBIT_SECRET_KEY environment variable not found. Please set it in .env file or environment.")?;

    set_access_key(&access_key);
    set_secret_key(&secret_key);

    Ok(())
}

/// 테스트용 초기화 함수입니다.
///
/// TEST_ACCESS_KEY와 TEST_SECRET_KEY 환경변수를 사용합니다.
pub fn init_for_test() -> Result<(), Box<dyn std::error::Error>> {
    load_dotenv();

    let access_key = std::env::var("TEST_ACCESS_KEY")
        .map_err(|_| "TEST_ACCESS_KEY environment variable not found. Please set it in .env file for testing.")?;

    let secret_key = std::env::var("TEST_SECRET_KEY")
        .map_err(|_| "TEST_SECRET_KEY environment variable not found. Please set it in .env file for testing.")?;

    set_access_key(&access_key);
    set_secret_key(&secret_key);

    Ok(())
}

/// function for setting secret key
pub fn set_secret_key(secret_key: &str) {
    envmnt::set("SECRET_KEY", secret_key);
}

/// function for setting access_key
pub fn set_access_key(access_key: &str) {
    envmnt::set("ACCESS_KEY", access_key);
}

// 테스트 전용 유틸리티 모듈 (cfg(test)로 테스트 시에만 컴파일)
#[cfg(test)]
pub mod test_utils;
