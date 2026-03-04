use domain::types::symbol::Symbol;
use reqwest::Method;

use crate::{
    client::BinanceClient,
    endpoints::{
        ACCOUNT_INFO, POSITION_MODE, POSITION_RISK,
    },
    errors::BinanceError,
    response_types::{FuturesAccountInfo, PositionModeResponse, PositionRisk},
    utils::build_query,
};

impl BinanceClient {
    pub async fn get_account_info(&self) -> Result<FuturesAccountInfo, BinanceError> {
        self.transport()
            .signed(Method::GET, ACCOUNT_INFO, String::new())
            .await
    }
    pub async fn get_position_mode(&self) -> Result<PositionModeResponse, BinanceError> {
        self.transport()
            .signed(Method::GET, POSITION_MODE, String::new())
            .await
    }

    pub async fn set_position_mode(
        &self,
        dual_side: bool,
    ) -> Result<PositionModeResponse, BinanceError> {
        let query = build_query(&[("dualSidePosition", dual_side.to_string())]);

        self.transport()
            .signed(Method::POST, POSITION_MODE, query)
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

        self.transport()
            .signed(Method::GET, POSITION_RISK, query)
            .await
    }
}
