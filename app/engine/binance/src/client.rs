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
