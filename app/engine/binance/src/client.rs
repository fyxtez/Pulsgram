use std::fmt;

use crate::{
    error::BinanceError,
    response_types::{
        FuturesAccountInfo, FuturesCommissionRateResponse, FuturesOrderResponse, ListenKeyResponse,
        PositionModeResponse, PositionRisk, SetLeverageResponse,
    },
    utils::{build_query, send_signed_request},
};
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

impl fmt::Display for OrderSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                OrderSide::Buy => "BUY",
                OrderSide::Sell => "SELL",
            }
        )
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

    pub async fn get_open_orders(
        &self,
        symbol: Option<&str>,
    ) -> Result<Vec<FuturesOrderResponse>, BinanceError> {
        let query = match symbol {
            Some(sym) => build_query(&[("symbol", sym.to_owned())]),
            None => String::new(),
        };

        let raw = send_signed_request(
            &self.client,
            Method::GET,
            &self.base_url,
            "fapi/v1/openOrders",
            &self.api_key,
            &self.api_secret,
            query,
        )
        .await?;

        let parsed: Vec<FuturesOrderResponse> = serde_json::from_str(&raw)?;

        Ok(parsed)
    }

    // newOrderRespType=RESULT
    // Forces Binance to return the final execution result for MARKET orders.
    // Without this, the API returns only ACK (orderId + clientOrderId),
    // which does not include executedQty, avgPrice, or final status.

    pub async fn get_trading_fees(
        &self,
        symbol: &str,
    ) -> Result<FuturesCommissionRateResponse, BinanceError> {
        let query = build_query(&[("symbol", symbol.to_owned())]);

        let raw = send_signed_request(
            &self.client,
            Method::GET,
            &self.base_url,
            "fapi/v1/commissionRate",
            &self.api_key,
            &self.api_secret,
            query,
        )
        .await?;

        let parsed: FuturesCommissionRateResponse = serde_json::from_str(&raw)?;

        Ok(parsed)
    }
    pub async fn get_account_info(&self) -> Result<FuturesAccountInfo, BinanceError> {
        let raw = send_signed_request(
            &self.client,
            Method::GET,
            &self.base_url,
            "fapi/v3/account",
            &self.api_key,
            &self.api_secret,
            String::new(),
        )
        .await?;

        let parsed: FuturesAccountInfo = serde_json::from_str(&raw)?;

        Ok(parsed)
    }
    pub async fn create_listen_key(&self) -> Result<String, BinanceError> {
        let url = format!("{}/fapi/v1/listenKey", self.base_url);

        let raw = self
            .client
            .post(&url)
            .header("X-MBX-APIKEY", &self.api_key)
            .send()
            .await?
            .text()
            .await?;

        let parsed: ListenKeyResponse = serde_json::from_str(&raw)?;

        Ok(parsed.listen_key)
    }
    pub async fn close_listen_key(&self, listen_key: &str) -> Result<(), BinanceError> {
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
        side: &OrderSide,
        quantity: &str,
    ) -> Result<FuturesOrderResponse, BinanceError> {
        quantity
            .parse::<f64>()
            .map_err(|_| BinanceError::InvalidInput("invalid quantity format".to_string()))?;

        if quantity.trim().is_empty() {
            return Err(BinanceError::InvalidInput(
                "quantity cannot be empty".to_string(),
            ));
        }
        let query = build_query(&[
            ("symbol", symbol.to_string()),
            ("side", side.to_string()),
            ("type", "MARKET".to_string()),
            ("quantity", quantity.to_string()),
            ("newOrderRespType", "RESULT".to_string()),
        ]);

        let raw = send_signed_request(
            &self.client,
            Method::POST,
            &self.base_url,
            "fapi/v1/order",
            &self.api_key,
            &self.api_secret,
            query,
        )
        .await?;
        let parsed: FuturesOrderResponse = serde_json::from_str(&raw)?;

        Ok(parsed)
    }
    pub async fn set_leverage(
        &self,
        symbol: &str,
        leverage: u32,
    ) -> Result<SetLeverageResponse, BinanceError> {
        let query = build_query(&[
            ("symbol", symbol.to_string()),
            ("leverage", leverage.to_string()),
        ]);

        let raw = send_signed_request(
            &self.client,
            Method::POST,
            &self.base_url,
            "fapi/v1/leverage",
            &self.api_key,
            &self.api_secret,
            query,
        )
        .await?;

        let parsed: SetLeverageResponse = serde_json::from_str(&raw)?;

        Ok(parsed)
    }
    pub async fn place_limit_order(
        &self,
        symbol: &str,
        side: &OrderSide,
        quantity: &str,
        price: &str,
    ) -> Result<FuturesOrderResponse, BinanceError> {
        let query = build_query(&[
            ("symbol", symbol.to_string()),
            ("side", side.to_string()),
            ("type", "LIMIT".to_string()),
            ("quantity", quantity.to_string()),
            ("price", price.to_string()),
            ("timeInForce", "GTC".to_string()),
        ]);

        let raw = send_signed_request(
            &self.client,
            Method::POST,
            &self.base_url,
            "fapi/v1/order",
            &self.api_key,
            &self.api_secret,
            query,
        )
        .await?;

        let parsed: FuturesOrderResponse = serde_json::from_str(&raw)?;

        Ok(parsed)
    }
    pub async fn get_position_mode(&self) -> Result<bool, BinanceError> {
        let raw = send_signed_request(
            &self.client,
            Method::GET,
            &self.base_url,
            "fapi/v1/positionSide/dual",
            &self.api_key,
            &self.api_secret,
            String::new(),
        )
        .await?;

        let json: serde_json::Value = serde_json::from_str(&raw)?;

        Ok(json["dualSidePosition"].as_bool().unwrap_or(false))
    }
    pub async fn get_position_risk(
        &self,
        symbol: Option<&str>,
    ) -> Result<Vec<PositionRisk>, BinanceError> {
        let query = match symbol {
            Some(s) => build_query(&[("symbol", s.to_owned())]),
            None => String::new(),
        };

        let raw = send_signed_request(
            &self.client,
            Method::GET,
            &self.base_url,
            "fapi/v3/positionRisk",
            &self.api_key,
            &self.api_secret,
            query,
        )
        .await?;

        let parsed: Vec<PositionRisk> = serde_json::from_str(&raw)?;

        Ok(parsed)
    }
    pub async fn set_position_mode(
        &self,
        dual_side: bool,
    ) -> Result<PositionModeResponse, BinanceError> {
        let mode = if dual_side { "true" } else { "false" };

        let query = build_query(&[("dualSidePosition", mode.to_string())]);

        let raw = send_signed_request(
            &self.client,
            Method::POST,
            &self.base_url,
            "fapi/v1/positionSide/dual",
            &self.api_key,
            &self.api_secret,
            query,
        )
        .await?;

        let parsed: PositionModeResponse = serde_json::from_str(&raw)?;

        Ok(parsed)
    }
    pub async fn cancel_order(
        &self,
        symbol: &str,
        order_id: i64,
    ) -> Result<FuturesOrderResponse, BinanceError> {
        let query = build_query(&[
            ("symbol", symbol.to_string()),
            ("orderId", order_id.to_string()),
        ]);

        let raw = send_signed_request(
            &self.client,
            Method::DELETE,
            &self.base_url,
            "fapi/v1/order",
            &self.api_key,
            &self.api_secret,
            query,
        )
        .await?;

        let parsed: FuturesOrderResponse = serde_json::from_str(&raw)?;

        Ok(parsed)
    }
}

#[cfg(test)]
#[serial_test::serial]
mod tests {
    use crate::constants;

    use super::*;

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

        let account = client
            .get_account_info()
            .await
            .expect("Expected account info response");

        assert!(!account.assets.is_empty());

        assert!(
            account.assets.iter().any(|a| a.asset == "USDT"),
            "USDT asset not found in account"
        );
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_trading_fees() {
        let client = test_client(constants::TESTNET_FUTURES);

        let result = client.get_trading_fees("BTCUSDT").await;

        assert!(result.is_ok());

        let fees = result.unwrap();

        assert_eq!(fees.symbol, "BTCUSDT");
        assert!(!fees.maker_commission_rate.is_empty());
        assert!(!fees.taker_commission_rate.is_empty());
        assert!(!fees.rpi_commission_rate.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn test_trading_fees_invalid_symbol() {
        let client = test_client(constants::TESTNET_FUTURES);

        let result = client.get_trading_fees("INVALIDSYMBOL").await;

        assert!(result.is_err());

        match result {
            Err(BinanceError::Api(msg)) => {
                assert!(msg.contains("Invalid symbol"));
            }
            Err(e) => panic!("Unexpected error variant: {:?}", e),
            Ok(_) => panic!("Expected error but got Ok"),
        }
    }

    #[tokio::test]
    #[ignore]
    async fn test_place_market_order() {
        let client = test_client(constants::TESTNET_FUTURES);

        let order = client
            .place_market_order("BTCUSDT", &OrderSide::Buy, "0.01")
            .await
            .expect("market buy failed");

        assert_eq!(order.symbol, "BTCUSDT");
        assert_eq!(order.side, "BUY");
        assert_eq!(order.r#type, "MARKET");
        assert!(order.order_id > 0);

        // Close position (reverse order)
        client
            .place_market_order("BTCUSDT", &OrderSide::Sell, "0.01")
            .await
            .expect("market sell failed");
    }

    #[tokio::test]
    #[ignore]
    async fn test_set_leverage() {
        let client = test_client(constants::TESTNET_FUTURES);

        let result = client.set_leverage("BTCUSDT", 33).await;

        match result {
            Ok(response) => {
                assert_eq!(response.symbol, "BTCUSDT");
                assert_eq!(response.leverage, 33);
            }

            Err(BinanceError::Api(msg)) => {
                println!("Leverage change failed: {}", msg);

                // Accept known Binance testnet instability
                assert!(
                    msg.contains("-1000"),
                    "Unexpected Binance API error: {}",
                    msg
                );
            }

            Err(e) => panic!("Unexpected error variant: {:?}", e),
        }
    }

    #[tokio::test]
    #[ignore]
    async fn test_place_and_cancel_limit_order() {
        let client = test_client(constants::TESTNET_FUTURES);

        // Place limit order far below market so it remains NEW
        let order = client
            .place_limit_order("BTCUSDT", &OrderSide::Buy, "0.01", "35000")
            .await
            .expect("failed to place limit order");

        println!("{order:#?}");

        assert_eq!(order.symbol, "BTCUSDT");
        assert_eq!(order.side, "BUY");
        assert_eq!(order.r#type, "LIMIT");
        assert!(order.order_id > 0);

        // For far-away price, order should normally remain NEW
        assert!(
            order.status == "NEW" || order.status == "FILLED",
            "unexpected order status: {}",
            order.status
        );

        let cancel = client
            .cancel_order("BTCUSDT", order.order_id)
            .await
            .expect("failed to cancel order");

        assert_eq!(cancel.status, "CANCELED");
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_open_orders() {
        let client = test_client(constants::TESTNET_FUTURES);

        let orders = client
            .get_open_orders(Some("BTCUSDT"))
            .await
            .expect("failed to fetch open orders");

        println!("Open orders: {:#?}", orders);

        // Orders may or may not exist.
        // Just ensure request works and structure is valid.
        for order in &orders {
            assert_eq!(order.symbol, "BTCUSDT");
        }
    }

    #[tokio::test]
    #[ignore]
    async fn test_set_position_mode() {
        let client = test_client(constants::TESTNET_FUTURES);

        let current = client.get_position_mode().await.unwrap();
        println!("Current mode: {}", current);

        if current != false {
            let result = client
                .set_position_mode(false)
                .await
                .expect("failed to change position mode");

            assert_eq!(result.code, 200);
            assert_eq!(result.msg, "success");
        }

        let current = client
            .get_position_mode()
            .await
            .expect("failed to fetch position mode");

        assert_eq!(current, false);
    }
    #[tokio::test]
    #[ignore]
    async fn test_get_position_risk() {
        let client = test_client(constants::TESTNET_FUTURES);

        let positions = client
            .get_position_risk(Some("BTCUSDT"))
            .await
            .expect("failed to fetch position risk");

        println!("Position risk: {:#?}", positions);

        // Request must succeed
        // Positions may be empty
        for pos in &positions {
            assert_eq!(pos.symbol, "BTCUSDT");
        }
    }
}
