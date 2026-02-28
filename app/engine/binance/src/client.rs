use std::collections::HashMap;

use crate::{
    constants::MAX_LEVERAGE,
    endpoints::{
        ACCOUNT_INFO, COMMISSION_RATE, EXCHANGE_INFO, LEVERAGE, LISTEN_KEY, OPEN_ORDERS, ORDER,
        POSITION_MODE, POSITION_RISK, TICKER_PRICE,
    },
    error::BinanceError,
    response_types::{
        ExchangeInfoResponse, FuturesAccountInfo, FuturesCommissionRateResponse,
        FuturesOrderResponse, ListenKeyResponse, PositionModeResponse, PositionRisk,
        SetLeverageResponse, TickerPriceResponse,
    },
    utils::{build_query, send_signed_request},
};
use domain::types::{
    order_side::OrderSide,
    symbol::{Symbol, SymbolFilters},
};
use reqwest::Method;
use serde::de::DeserializeOwned;

pub struct BinanceClient {
    client: reqwest::Client,
    base_url: String,
    api_key: String,
    api_secret: String,
    symbol_filters: HashMap<Symbol, SymbolFilters>,
}

impl BinanceClient {
    pub fn new(client: reqwest::Client, base_url: &str, api_key: &str, api_secret: &str) -> Self {
        Self {
            client,
            base_url: base_url.to_string(),
            api_key: api_key.to_string(),
            api_secret: api_secret.to_string(),
            symbol_filters: HashMap::new(),
        }
    }

    pub fn set_symbol_filters(&mut self, filters: HashMap<Symbol, SymbolFilters>) {
        self.symbol_filters = filters;
    }

    pub fn get_filters(&self, symbol: &Symbol) -> Option<&SymbolFilters> {
        self.symbol_filters.get(symbol)
    }
    pub fn min_quantity(&self, symbol: Symbol) -> Result<f64, BinanceError> {
        self.symbol_filters
            .get(&symbol)
            .map(|f| f.min_qty)
            .ok_or_else(|| BinanceError::InvalidInput(format!("Unknown symbol: {}", symbol)))
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
        let response = send_signed_request(
            &self.client,
            method,
            &self.base_url,
            endpoint,
            &self.api_key,
            &self.api_secret,
            query,
        )
        .await?;

        let text = response.text().await?;

        let value: serde_json::Value = serde_json::from_str(&text)?;

        // Detect Binance error first
        if let Some(code) = value.get("code").and_then(|c| c.as_i64())
            && code < 0
        {
            let api_err = serde_json::from_value(value)?;
            return Err(BinanceError::Api(api_err));
        }

        let parsed = serde_json::from_value::<T>(value)?;
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

        if let Some(q) = query
            && !q.is_empty()
        {
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
        symbol: Option<Symbol>,
    ) -> Result<Vec<FuturesOrderResponse>, BinanceError> {
        let query = match symbol {
            Some(sym) => build_query(&[("symbol", sym.to_string())]),
            None => String::new(),
        };
        self.signed_request(Method::GET, OPEN_ORDERS, query).await
    }

    pub async fn get_trading_fees(
        &self,
        symbol: Symbol,
    ) -> Result<FuturesCommissionRateResponse, BinanceError> {
        let query = build_query(&[("symbol", symbol.to_string())]);

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

    fn format_quantity(&self, quantity: f64, step_size: f64) -> String {
        let precision = step_size
            .to_string()
            .split('.')
            .nth(1)
            .map(|s| s.len())
            .unwrap_or(0);

        format!("{:.*}", precision, quantity)
    }

    pub async fn place_minimum_market_order(
        &self,
        symbol: Symbol,
        side: &OrderSide,
    ) -> Result<FuturesOrderResponse, BinanceError> {
        let filters = self
            .symbol_filters
            .get(&symbol)
            .ok_or_else(|| BinanceError::InvalidInput(format!("Unknown symbol: {}", symbol)))?;

        let current_price = self.get_current_price(symbol).await?;

        let min_notional_qty = filters.min_notional / current_price;

        let raw = filters.min_qty.max(min_notional_qty);

        let steps = (raw / filters.step_size).ceil();
        let qty = steps * filters.step_size;

        self.place_market_order(symbol, side, qty).await
    }

    pub async fn place_market_order(
        &self,
        symbol: Symbol,
        side: &OrderSide,
        quantity: f64,
    ) -> Result<FuturesOrderResponse, BinanceError> {
        let filters = self
            .symbol_filters
            .get(&symbol)
            .ok_or(BinanceError::InvalidInput(format!(
                "Unknown symbol: {}",
                symbol
            )))?;
        if quantity < filters.min_qty {
            return Err(BinanceError::InvalidInput(format!(
                "Quantity {} below min_qty {}",
                quantity, filters.min_qty
            )));
        }

        let aligned_qty = self.align_to_step(quantity, filters.step_size);

        if aligned_qty <= 0.0 {
            return Err(BinanceError::InvalidInput(format!(
                "Quantity {} invalid after step alignment",
                quantity
            )));
        }
        let quantity_str = self.format_quantity(aligned_qty, filters.step_size);

        let query = build_query(&[
            ("symbol", symbol.to_string()),
            ("side", side.to_string()),
            ("type", "MARKET".to_string()),
            ("quantity", quantity_str),
            ("newOrderRespType", "RESULT".to_string()),
        ]);

        self.signed_request(Method::POST, ORDER, query).await
    }

    fn align_to_step(&self, quantity: f64, step: f64) -> f64 {
        (quantity / step).floor() * step
    }

    pub async fn get_exchange_info(&self) -> Result<ExchangeInfoResponse, BinanceError> {
        self.api_key_request(Method::GET, EXCHANGE_INFO, None).await
    }

    pub async fn set_leverage(
        &self,
        symbol: Symbol,
        leverage: u32,
    ) -> Result<SetLeverageResponse, BinanceError> {
        if leverage == 0 || leverage > MAX_LEVERAGE {
            return Err(BinanceError::InvalidInput(format!(
                "Invalid leverage {}. Allowed range: 1-{}",
                leverage, MAX_LEVERAGE
            )));
        }

        let query = build_query(&[
            ("symbol", symbol.to_string()),
            ("leverage", leverage.to_string()),
        ]);

        self.signed_request(Method::POST, LEVERAGE, query).await
    }

    pub async fn place_limit_order(
        &self,
        symbol: Symbol,
        side: &OrderSide,
        quantity: f64,
        price: f64,
    ) -> Result<FuturesOrderResponse, BinanceError> {
        let filters = self
            .symbol_filters
            .get(&symbol)
            .ok_or(BinanceError::InvalidInput(format!(
                "Unknown symbol: {}",
                symbol
            )))?;

        if quantity < filters.min_qty {
            return Err(BinanceError::InvalidInput(format!(
                "Quantity {} below min_qty {}",
                quantity, filters.min_qty
            )));
        }

        let aligned_qty = self.align_to_step(quantity, filters.step_size);
        if aligned_qty <= 0.0 {
            return Err(BinanceError::InvalidInput(
                "Quantity invalid after alignment".into(),
            ));
        }

        let aligned_price = self.align_to_step(price, filters.tick_size);
        if aligned_price <= 0.0 {
            return Err(BinanceError::InvalidInput(
                "Price invalid after alignment".into(),
            ));
        }

        let quantity_str = self.format_quantity(aligned_qty, filters.step_size);
        let price_str = self.format_quantity(aligned_price, filters.tick_size);

        let query = build_query(&[
            ("symbol", symbol.to_string()),
            ("side", side.to_string()),
            ("type", "LIMIT".to_string()),
            ("quantity", quantity_str),
            ("price", price_str),
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
        symbol: Option<Symbol>,
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
        symbol: Symbol,
        order_id: i64,
    ) -> Result<FuturesOrderResponse, BinanceError> {
        let query = build_query(&[
            ("symbol", symbol.to_string()),
            ("orderId", order_id.to_string()),
        ]);

        self.signed_request(Method::DELETE, ORDER, query).await
    }

    pub async fn close_percentage(&self, symbol: Symbol, percent: f64) -> Result<(), BinanceError> {
        if percent <= 0.0 || percent > 100.0 {
            return Err(BinanceError::InvalidInput(
                "percent must be between 0 and 100".into(),
            ));
        }

        let positions = self.get_position_risk(Some(symbol)).await?;

        let pos = positions
            .into_iter()
            .find(|p| p.symbol == symbol.to_string())
            .ok_or_else(|| BinanceError::InvalidInput("Position not found".into()))?;

        let amt: f64 = pos.position_amt.parse().unwrap_or(0.0);

        if amt == 0.0 {
            return Ok(());
        }

        let raw = amt.abs() * percent / 100.0;

        if raw <= 0.0 {
            return self.close_full_position(symbol).await;
        }

        let side = if amt > 0.0 {
            OrderSide::Sell
        } else {
            OrderSide::Buy
        };

        self.place_market_order(symbol, &side, raw).await?;

        Ok(())
    }

    pub async fn get_current_price(&self, symbol: Symbol) -> Result<f64, BinanceError> {
        let query = build_query(&[("symbol", symbol.to_string())]);

        let resp: TickerPriceResponse = self
            .api_key_request(Method::GET, TICKER_PRICE, Some(query))
            .await?;

        let price = resp
            .price
            .parse::<f64>()
            .map_err(|_| BinanceError::InvalidInput("Invalid ticker price".into()))?;

        Ok(price)
    }

    pub async fn close_full_position(&self, symbol: Symbol) -> Result<(), BinanceError> {
        let positions = self.get_position_risk(Some(symbol)).await?;

        let pos = positions
            .into_iter()
            .find(|p| p.symbol == symbol.to_string())
            .ok_or_else(|| BinanceError::InvalidInput("Position not found".into()))?;

        let amt: f64 = pos
            .position_amt
            .parse()
            .map_err(|_| BinanceError::InvalidInput("Invalid position amount".into()))?;

        if amt == 0.0 {
            return Ok(());
        }

        let side = if amt > 0.0 {
            OrderSide::Sell
        } else {
            OrderSide::Buy
        };

        self.place_market_order(symbol, &side, amt.abs()).await?;

        Ok(())
    }
}
