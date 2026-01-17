use std::env;
use std::error::Error;
// use std::fs;
// use std::path::Path;

// use grammers_session::storages::SqliteSession;


#[derive(Debug)]
pub struct ConfigData {
    pub api_id: i32,
    pub api_hash: String,
    pub phone_number: String,
    pub password: String,
}

pub fn load_tg_client_config() -> Result<ConfigData, Box<dyn Error>> {
    let api_id_str = env::var("API_ID")
        .map_err(|_| "Missing environment variable: API_ID")?;
    let api_id = api_id_str.parse()
        .map_err(|_| "Failed to parse API_ID as i32")?;
    let api_hash = env::var("API_HASH")
        .map_err(|_| "Missing environment variable: API_HASH")?;
    let phone_number = env::var("PHONE_NUMBER")
        .map_err(|_| "Missing environment variable: PHONE_NUMBER")?;
    let password = env::var("PASSWORD")
        .map_err(|_| "Missing environment variable: PASSWORD")?;

    Ok(ConfigData {
        api_id,
        api_hash,
        phone_number,
        password,
    })
}

// pub async fn load_or_create_session(session_file: &str) -> Result<SqliteSession, Box<dyn Error>> {
//        Ok(SqliteSession::open(session_file)?)
// }

// pub fn save_session(session_file: &str, session_data: Vec<u8>) -> Result<(), Box<dyn Error>> {
//     fs::write(Path::new(session_file), session_data)?;
//     Ok(())
// }
