use domain::types::symbol::Symbol;
use reqwest::Method;

use crate::{
    client::BinanceClient,
    endpoints::{EXCHANGE_INFO, TICKER_PRICE},
    errors::BinanceError,
    response_types::{ExchangeInfoResponse, TickerPriceResponse},
    utils::build_query,
};

impl BinanceClient {
    pub async fn get_exchange_info(&self) -> Result<ExchangeInfoResponse, BinanceError> {
        self.transport()
            .api_key(Method::GET, EXCHANGE_INFO, None)
            .await
    }

    pub async fn get_current_price(&self, symbol: Symbol) -> Result<f64, BinanceError> {
        let query = build_query(&[("symbol", symbol.to_string())]);

        let resp: TickerPriceResponse = self
            .transport()
            .api_key(Method::GET, TICKER_PRICE, Some(query))
            .await?;

        let price = resp
            .price
            .parse::<f64>()
            .map_err(|_| BinanceError::InvalidInput("Invalid ticker price".into()))?;

        Ok(price)
    }
}
