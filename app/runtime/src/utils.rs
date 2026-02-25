use crate::error::AppError;

pub fn create_reqwest_client() -> Result<reqwest::Client, AppError> {
    let client = reqwest::Client::builder()
        .user_agent("Pulsgram/1.0")
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    Ok(client)
}
