use domain::types::{order_side::OrderSide, symbol::Symbol};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DevTradeApprovedRequest {
    pub symbol: Symbol,
    pub side: OrderSide,
    pub entry: f64,
    pub stop_loss: f64,
    pub targets: Vec<f64>,
    pub timeframe: String,
}
