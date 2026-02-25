use std::env;

use crate::errors::TelegramError;

#[derive(Debug)]
pub struct ConfigData {
    pub api_id: i32,
    pub api_hash: String,
    pub phone_number: String,
    pub password: String,
}

pub fn load_tg_client_config(
    api_id: &str,
    api_hash: &str,
    phone_number: &str,
    password: &str,
) -> Result<ConfigData, TelegramError> {
    let api_id_str = env::var(api_id)
        .map_err(|e| TelegramError::EnvVar {
            name: api_id.to_string(),
            source: e,
        })?;

    let api_id = api_id_str
        .parse::<i32>()
        .map_err(|e| TelegramError::ParseInt {
            name: api_id.to_string(),
            source: e,
        })?;

    let api_hash = env::var(api_hash)
        .map_err(|e| TelegramError::EnvVar {
            name: api_hash.to_string(),
            source: e,
        })?;

    let phone_number = env::var(phone_number)
        .map_err(|e| TelegramError::EnvVar {
            name: phone_number.to_string(),
            source: e,
        })?;

    let password = env::var(password)
        .map_err(|e| TelegramError::EnvVar {
            name: password.to_string(),
            source: e,
        })?;

    Ok(ConfigData {
        api_id,
        api_hash,
        phone_number,
        password,
    })
}