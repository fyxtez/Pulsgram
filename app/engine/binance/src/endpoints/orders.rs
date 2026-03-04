use domain::types::{order_side::OrderSide, symbol::Symbol};
use reqwest::Method;

use crate::{
    client::BinanceClient,
    constants::MAX_LEVERAGE,
    endpoints::{LEVERAGE, OPEN_ORDERS, ORDER},
    errors::BinanceError,
    response_types::{FuturesOrderResponse, SetLeverageResponse},
    utils::build_query,
};

impl BinanceClient {
    pub async fn get_open_orders(
        &self,
        symbol: Option<Symbol>,
    ) -> Result<Vec<FuturesOrderResponse>, BinanceError> {
        let query = match symbol {
            Some(sym) => build_query(&[("symbol", sym.to_string())]),
            None => String::new(),
        };
        self.transport()
            .signed(Method::GET, OPEN_ORDERS, query)
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

        self.transport().signed(Method::DELETE, ORDER, query).await
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

        self.transport().signed(Method::POST, LEVERAGE, query).await
    }

    pub async fn place_market_order_raw(
        &self,
        symbol: Symbol,
        side: &OrderSide,
        quantity: String,
    ) -> Result<FuturesOrderResponse, BinanceError> {
        let query = build_query(&[
            ("symbol", symbol.to_string()),
            ("side", side.to_string()),
            ("type", "MARKET".to_string()),
            ("quantity", quantity),
            ("newOrderRespType", "RESULT".to_string()),
        ]);

        self.transport().signed(Method::POST, ORDER, query).await
    }

    pub async fn place_limit_order_raw(
        &self,
        symbol: Symbol,
        side: &OrderSide,
        quantity: String,
        price: String,
    ) -> Result<FuturesOrderResponse, BinanceError> {
        let query = build_query(&[
            ("symbol", symbol.to_string()),
            ("side", side.to_string()),
            ("type", "LIMIT".to_string()),
            ("quantity", quantity),
            ("price", price),
            ("timeInForce", "GTC".to_string()),
        ]);

        self.transport().signed(Method::POST, ORDER, query).await
    }
}
