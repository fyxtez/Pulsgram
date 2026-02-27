use std::fmt;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum OrderSide {
    Buy, // LONG
    Sell, // SHORT
}
impl From<bool> for OrderSide {
    fn from(is_long: bool) -> Self {
        if is_long {
            OrderSide::Buy
        } else {
            OrderSide::Sell
        }
    }
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

#[derive(Debug, Clone)]
pub struct TradeIntent {
    pub intent_id: Uuid,
    pub symbol: String,
    pub side: OrderSide,
}

impl TradeIntent {
    pub fn new(symbol: String, side: OrderSide) -> Self {
        Self {
            intent_id: Uuid::new_v4(),
            symbol,
            side,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TradeRejected {
    pub intent_id: Uuid,
    pub symbol: String,
    pub reason: TradeRejectionReason,
}

#[derive(Debug, Clone)]
pub struct TradeApproved {
    pub intent_id: Uuid,
    pub symbol: String,
    pub side: OrderSide,
}

#[derive(Debug, Clone)]
pub enum TradeRejectionReason {
    RiskRejected,
    InvalidSignal,
    InsufficientBalance,
    Other(String),
}
