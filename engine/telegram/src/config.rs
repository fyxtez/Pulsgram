use std::env;
use std::error::Error;

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
) -> Result<ConfigData, Box<dyn Error>> {
    let api_id_str = env::var(api_id).map_err(|_| format!("Missing env var: {}", api_id))?;
    let api_id = api_id_str
        .parse::<i32>()
        .map_err(|_| format!("Failed to parse {} as i32", api_id))?;
    let api_hash = env::var(api_hash).map_err(|_| format!("Missing env var: {}", api_hash))?;
    let phone_number =
        env::var(phone_number).map_err(|_| format!("Missing env var: {}", phone_number))?;
    let password = env::var(password).map_err(|_| format!("Missing env var: {}", password))?;

    Ok(ConfigData {
        api_id,
        api_hash,
        phone_number,
        password,
    })
}
