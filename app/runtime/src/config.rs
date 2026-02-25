use std::env;
use dotenv::dotenv;

use crate::error::AppError;

#[derive(Debug)]
pub struct Config {
    pub kol_follows_chat_id: i64,
    pub errors_peer_id: i64,
    pub kol_follows_test_chat_id: i64,
    pub perp_signals_chat_id: i64,
    pub perp_signals_test_chat_id: i64,
    pub perp_kols_chat_id: i64,
    pub perp_kols_test_chat_id: i64,
    pub perp_kols_usernames: Vec<String>,
    pub lcs_user_id: i64,
    pub rs_user_id: i64,
    pub binance_api_key: String,
    pub binance_api_secret: String,
}


fn required_env_string(key: &str) -> Result<String, AppError> {
    Ok(env::var(key)?)
}

fn required_env_i64(key: &str) -> Result<i64, AppError> {
    let val = env::var(key)?;
    val.parse::<i64>()
        .map_err(|e| AppError::Other(format!("Invalid value for {key}: {e}")))
}

impl Config {
    pub fn from_env(use_binance_testnet:bool) -> Result<Self, AppError> {
        dotenv().ok();

        let (api_key_var, api_secret_var) = if use_binance_testnet {
            ("BINANCE_API_KEY_TEST", "BINANCE_API_SECRET_TEST")
        } else {
            ("BINANCE_API_KEY", "BINANCE_API_SECRET")
        };

        Ok(Self {
            kol_follows_chat_id: required_env_i64("KOL_FOLLOWS_CHAT_ID")?,
            errors_peer_id: required_env_i64("ERRORS_PEER_ID")?,
            kol_follows_test_chat_id: required_env_i64("KOL_FOLLOWS_TEST_CHAT_ID")?,
            perp_signals_chat_id: required_env_i64("PERP_SIGNALS_CHAT_ID")?,
            perp_signals_test_chat_id: required_env_i64("PERP_SIGNALS_TEST_CHAT_ID")?,
            perp_kols_chat_id: required_env_i64("PERP_KOLS_CHAT_ID")?,
            perp_kols_test_chat_id: required_env_i64("PERP_KOLS_TEST_CHAT_ID")?,
            perp_kols_usernames: required_env_string("PERP_KOLS_USERNAMES")?
                .split(',')
                .map(|s| s.to_string())
                .collect(),
            lcs_user_id: required_env_i64("LCS_USER_ID")?,
            rs_user_id: required_env_i64("RS_USER_ID")?,
            binance_api_key: required_env_string(api_key_var)?,
            binance_api_secret: required_env_string(api_secret_var)?,
        })
    }
}
