use crate::utils::send_signed_request;
use reqwest::Method;

pub struct BinanceClient {
    client: reqwest::Client,
    base_url: String,
    api_key: String,
    api_secret: String,
}

pub enum OrderSide {
    Buy,
    Sell,
}

impl OrderSide {
    fn as_str(&self) -> &'static str {
        match self {
            OrderSide::Buy => "BUY",
            OrderSide::Sell => "SELL",
        }
    }
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
    pub async fn create_listen_key(&self) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("{}/fapi/v1/listenKey", self.base_url);
        let resp = self
            .client
            .post(&url)
            .header("X-MBX-APIKEY", &self.api_key)
            .send()
            .await?
            .text()
            .await?;

        let v: serde_json::Value = serde_json::from_str(&resp)?;
        v["listenKey"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| format!("no listenKey in response: {resp}").into())
    }
    pub async fn close_listen_key(
        &self,
        listen_key: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!(
            "{}/fapi/v1/listenKey?listenKey={}",
            self.base_url, listen_key
        );

        self.client
            .delete(&url)
            .header("X-MBX-APIKEY", &self.api_key)
            .send()
            .await?;

        Ok(())
    }
    pub async fn place_market_order(
        &self,
        symbol: &str,
        side:&OrderSide,
        quantity: f64,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let query = format!(
            "symbol={}&side={}&type=MARKET&quantity={}",
            symbol, side.as_str(), quantity
        );

        send_signed_request(
            &self.client,
            Method::POST,
            &self.base_url,
            "fapi/v1/order",
            &self.api_key,
            &self.api_secret,
            query,
        )
        .await
    }
    pub async fn set_leverage(
        &self,
        symbol: &str,
        leverage: u32,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let query = format!("symbol={}&leverage={}", symbol, leverage);

        send_signed_request(
            &self.client,
            Method::POST,
            &self.base_url,
            "fapi/v1/leverage",
            &self.api_key,
            &self.api_secret,
            query,
        )
        .await
    }
    pub async fn place_limit_order(
        &self,
        symbol: &str,
        side: &OrderSide,
        quantity: f64,
        price: f64,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let query = format!(
            "symbol={}&side={}&type=LIMIT&quantity={}&price={}&timeInForce=GTC",
            symbol, side.as_str(), quantity, price
        );

        send_signed_request(
            &self.client,
            Method::POST,
            &self.base_url,
            "fapi/v1/order",
            &self.api_key,
            &self.api_secret,
            query,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use crate::constants;

    use super::*;
    use std::env;

    fn test_client(url: &str) -> BinanceClient {
        dotenv::from_filename("app/.env").ok();

        let api_key = std::env::var("BINANCE_API_KEY_TEST").expect("Set BINANCE_API_KEY_TEST");

        let api_secret =
            std::env::var("BINANCE_API_SECRET_TEST").expect("Set BINANCE_API_SECRET_TEST");

        BinanceClient::new(reqwest::Client::new(), url, &api_key, &api_secret)
    }

    #[tokio::test]
    #[ignore]
    async fn test_listener_key() {
        let client = test_client(constants::TESTNET_FUTURES);

        let result = client.create_listen_key().await;

        assert!(result.is_ok());

        let listen_key = result.unwrap();

        let close_result = client.close_listen_key(&listen_key).await;

        assert!(close_result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_account_info() {
        let client = test_client(constants::TESTNET_FUTURES);

        let result = client.get_account_info().await;

        assert!(result.is_ok());

        let body = result.unwrap();
        assert!(body.contains("assets"));
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_trading_fees() {
        let client = test_client(constants::TESTNET_FUTURES);

        let result = client.get_trading_fees("BTCUSDT").await;

        assert!(result.is_ok());

        let body = result.unwrap();
        assert!(body.contains("makerCommissionRate"));
    }

    #[tokio::test]
    #[ignore]
    async fn test_trading_fees_invalid_symbol() {
        let client = test_client(constants::TESTNET_FUTURES);

        let result = client.get_trading_fees("INVALIDSYMBOL").await;

        assert!(result.is_ok());
        assert!(result.unwrap().contains("Invalid symbol"));
    }

    #[tokio::test]
    #[ignore]
    async fn test_place_market_order() {
        let client = test_client(constants::TESTNET_FUTURES);

        let result = client.place_market_order("BTCUSDT", &OrderSide::Buy, 0.01).await;

        println!("{result:?}");

        assert!(result.is_ok());

        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();

        assert_eq!(json["symbol"], "BTCUSDT");
        assert_eq!(json["side"], "BUY");
        assert_eq!(json["type"], "MARKET");

        let order_id = json["orderId"].as_u64().expect("orderId should be a u64");

        assert!(order_id > 0);

        client
            .place_market_order("BTCUSDT", &OrderSide::Sell, 0.01)
            .await
            .unwrap();
    }

    #[tokio::test]
    #[ignore]
    async fn test_set_leverage() {
        let client = test_client(constants::TESTNET_FUTURES);

        let result = client.set_leverage("BTCUSDT", 10).await.unwrap();

        let json: serde_json::Value = serde_json::from_str(&result).unwrap();

        assert_eq!(json["symbol"], "BTCUSDT");
        assert_eq!(json["leverage"], 10);
    }

    #[tokio::test]
    #[ignore]
    async fn test_place_and_cancel_limit_order() {
        let client = test_client(constants::TESTNET_FUTURES);

        // Place limit order far below market so it stays open
        let result = client
            .place_limit_order("BTCUSDT", &OrderSide::Buy, 0.01, 35000.0)
            .await
            .unwrap();

        let json: serde_json::Value = serde_json::from_str(&result).unwrap();

        let order_id = json["orderId"].as_u64().unwrap();
        assert!(order_id > 0);

        // TODO

        // let cancel = client.cancel_order("BTCUSDT", order_id).await.unwrap();

        // let cancel_json: serde_json::Value = serde_json::from_str(&cancel).unwrap();

        // assert_eq!(cancel_json["status"], "CANCELED");
    }
}
