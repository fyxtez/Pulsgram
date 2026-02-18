use crate::utils::send_signed_request;
use reqwest::Method;

pub struct BinanceClient {
    client: reqwest::Client,
    base_url: String,
    api_key: String,
    api_secret: String,
}

impl BinanceClient {
    pub fn new(client: reqwest::Client, base_url: &str, api_key: &str, api_secret: &str) -> Self {
        Self {
            client,
            base_url: base_url.to_string(),
            api_key: api_key.to_string(),
            api_secret: api_secret.to_string(),
        }
    }

    pub async fn get_trading_fees(
        &self,
        symbol: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let query = format!("symbol={}", symbol);

        send_signed_request(
            &self.client,
            Method::GET,
            &self.base_url,
            "fapi/v1/commissionRate",
            &self.api_key,
            &self.api_secret,
            query,
        )
        .await
    }

    pub async fn get_account_info(&self) -> Result<String, Box<dyn std::error::Error>> {
        send_signed_request(
            &self.client,
            Method::GET,
            &self.base_url,
            "fapi/v2/account",
            &self.api_key,
            &self.api_secret,
            String::new(),
        )
        .await
    }
}
