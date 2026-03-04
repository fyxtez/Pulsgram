use domain::types::symbol::Symbol;
use reqwest::Method;

use crate::{
    client::BinanceClient, endpoints::COMMISSION_RATE, errors::BinanceError,
    response_types::FuturesCommissionRateResponse, utils::build_query,
};

impl BinanceClient {
    pub async fn get_trading_fees(
        &self,
        symbol: Symbol,
    ) -> Result<FuturesCommissionRateResponse, BinanceError> {
        let query = build_query(&[("symbol", symbol.to_string())]);

        self.transport()
            .signed(Method::GET, COMMISSION_RATE, query)
            .await
    }
}
