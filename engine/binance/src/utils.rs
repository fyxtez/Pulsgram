use hmac::{Hmac, Mac};
use sha2::Sha256;

#[derive(Debug)]
pub struct BinanceEnv {
    pub api_key: String,
    pub api_secret: String,
}

fn get_timestamp() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

fn create_signature(query_string: &str, secret: &str) -> String {
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(query_string.as_bytes());
    hex::encode(mac.finalize().into_bytes())
}

use reqwest::Method;

pub async fn send_signed_request(
    client: &reqwest::Client,
    method: Method,
    base_url: &str,
    endpoint: &str,
    api_key: &str,
    api_secret: &str,
    mut query_string: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let timestamp = get_timestamp();

    if !query_string.is_empty() {
        query_string.push('&');
    }

    query_string.push_str(&format!("timestamp={}", timestamp));

    let signature = create_signature(&query_string, api_secret);

    let url: String = format!("{}/{}", base_url, endpoint);

    let request = match method {
        Method::GET => client.request(
            method,
            format!("{}?{}&signature={}", url, query_string, signature),
        ),
        _ => client
            .request(method, url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(format!("{}&signature={}", query_string, signature)),
    };

    let response = request.header("X-MBX-APIKEY", api_key).send().await?;

    let text = response.text().await?;

    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
        return Ok(serde_json::to_string_pretty(&json)?);
    }

    Ok(text)
}

pub fn load_env_vars(
    api_key: &str,
    api_secret: &str,
) -> Result<BinanceEnv, Box<dyn std::error::Error>> {
    let api_key = std::env::var(api_key).map_err(|_| "BINANCE_API_KEY_TEST not set")?;

    let api_secret = std::env::var(api_secret).map_err(|_| "BINANCE_API_SECRET_TEST not set")?;

    Ok(BinanceEnv {
        api_key,
        api_secret,
    })
}
