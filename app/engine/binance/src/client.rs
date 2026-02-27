use crate::{
    endpoints::{
        ACCOUNT_INFO, COMMISSION_RATE, LEVERAGE, LISTEN_KEY, OPEN_ORDERS, ORDER, POSITION_MODE,
        POSITION_RISK,
    },
    error::BinanceError,
    response_types::{
        FuturesAccountInfo, FuturesCommissionRateResponse, FuturesOrderResponse, ListenKeyResponse,
        PositionModeResponse, PositionRisk, SetLeverageResponse,
    },
    types::OrderSide,
    utils::{build_query, send_signed_request},
};
use reqwest::Method;
use serde::de::DeserializeOwned;

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

    async fn signed_request<T>(
        &self,
        method: Method,
        endpoint: &str,
        query: String,
    ) -> Result<T, BinanceError>
    where
        T: DeserializeOwned,
    {
        let raw = send_signed_request(
            &self.client,
            method,
            &self.base_url,
            endpoint,
            &self.api_key,
            &self.api_secret,
            query,
        )
        .await?;

        let parsed = serde_json::from_str::<T>(&raw)?;
        Ok(parsed)
    }

    async fn api_key_request<T>(
        &self,
        method: Method,
        endpoint: &str,
        query: Option<String>,
    ) -> Result<T, BinanceError>
    where
        T: DeserializeOwned,
    {
        let mut url = format!("{}/{}", self.base_url, endpoint);

        if let Some(q) = query && 
            !q.is_empty() {
                url.push('?');
                url.push_str(&q);
        }

        let response = self
            .client
            .request(method, &url)
            .header("X-MBX-APIKEY", &self.api_key)
            .send()
            .await?;

        let response = response.error_for_status()?; // 👈 important

        let raw = response.text().await?;

        let parsed = serde_json::from_str::<T>(&raw)?;
        Ok(parsed)
    }

    pub async fn get_open_orders(
        &self,
        symbol: Option<&str>,
    ) -> Result<Vec<FuturesOrderResponse>, BinanceError> {
        let query = match symbol {
            Some(sym) => build_query(&[("symbol", sym.to_owned())]),
            None => String::new(),
        };
        self.signed_request(Method::GET, OPEN_ORDERS, query).await
    }

    pub async fn get_trading_fees(
        &self,
        symbol: &str,
    ) -> Result<FuturesCommissionRateResponse, BinanceError> {
        let query = build_query(&[("symbol", symbol.to_owned())]);

        self.signed_request(Method::GET, COMMISSION_RATE, query)
            .await
    }

    pub async fn get_account_info(&self) -> Result<FuturesAccountInfo, BinanceError> {
        self.signed_request(Method::GET, ACCOUNT_INFO, String::new())
            .await
    }

    pub async fn create_listen_key(&self) -> Result<String, BinanceError> {
        let resp: ListenKeyResponse = self.api_key_request(Method::POST, LISTEN_KEY, None).await?;

        Ok(resp.listen_key)
    }

    pub async fn close_listen_key(&self, listen_key: &str) -> Result<(), BinanceError> {
        let query = build_query(&[("listenKey", listen_key.to_string())]);

        let _: serde_json::Value = self
            .api_key_request(Method::DELETE, LISTEN_KEY, Some(query))
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

        self.signed_request(Method::POST, ORDER, query).await
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

        self.signed_request(Method::POST, LEVERAGE, query).await
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

        self.signed_request(Method::POST, ORDER, query).await
    }

    pub async fn get_position_mode(&self) -> Result<PositionModeResponse, BinanceError> {
        self.signed_request(Method::GET, POSITION_MODE, String::new())
            .await
    }

    // Get current position information(only symbol that has position or open orders will be returned).
    pub async fn get_position_risk(
        &self,
        symbol: Option<&str>,
    ) -> Result<Vec<PositionRisk>, BinanceError> {
        let query = match symbol {
            Some(s) => build_query(&[("symbol", s.to_string())]),
            None => String::new(),
        };

        self.signed_request(Method::GET, POSITION_RISK, query).await
    }

    pub async fn set_position_mode(
        &self,
        dual_side: bool,
    ) -> Result<PositionModeResponse, BinanceError> {
        let query = build_query(&[("dualSidePosition", dual_side.to_string())]);

        self.signed_request(Method::POST, POSITION_MODE, query)
            .await
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

        self.signed_request(Method::DELETE, ORDER, query).await
    }
}
