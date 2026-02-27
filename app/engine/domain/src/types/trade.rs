use uuid::Uuid;

use crate::types::{order_side::OrderSide, symbol::Symbol, trade_intent::TradeIntent};

#[derive(Debug, Clone)]
pub struct TradeRejected {
    pub intent_id: Uuid,
    pub symbol: Symbol,
    pub reason: TradeRejectionReason,
}

#[derive(Debug, Clone)]
pub struct TradeApproved {
    pub intent_id: Uuid,
    pub symbol: Symbol,
    pub side: OrderSide,
    pub entry: f64,
    pub targets: Vec<f64>,
    pub timeframe: String,
    pub stop_loss: f64,
}

impl From<TradeIntent> for TradeApproved {
    fn from(intent: TradeIntent) -> Self {
        Self {
            intent_id: intent.intent_id,
            symbol: intent.symbol,
            side: intent.side,
            entry: intent.entry,
            targets: intent.targets,
            timeframe: intent.timeframe,
            stop_loss: intent.stop_loss,
        }
    }
}

#[derive(Debug, Clone)]
pub enum TradeRejectionReason {
    RiskRejected,
    InvalidSignal,
    InsufficientBalance,
    Other(String),
}
