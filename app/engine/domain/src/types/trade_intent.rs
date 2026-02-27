use core::fmt;

use uuid::Uuid;

use crate::types::{order_side::OrderSide, symbol::Symbol};

#[derive(Debug, Clone)]
pub struct TradeIntent {
    pub intent_id: Uuid,
    pub symbol: Symbol,
    pub side: OrderSide,
    pub entry: f64,
    pub targets: Vec<f64>,
    pub timeframe: String,
    pub stop_loss: f64,
}

#[derive(Debug)]
pub enum TradeIntentError {
    MissingSide,
    MissingEntry,
    MissingTargets,
    MissingTimeframe,
    MissingStopLoss,
}

impl fmt::Display for TradeIntentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TradeIntentError::MissingSide => write!(f, "Side is missing"),
            TradeIntentError::MissingEntry => write!(f, "Entry price is missing"),
            TradeIntentError::MissingTargets => write!(f, "Targets are missing"),
            TradeIntentError::MissingTimeframe => write!(f, "Timeframe is missing"),
            TradeIntentError::MissingStopLoss => write!(f, "Stop loss is missing"),
        }
    }
}

pub struct TradeIntentBuilder {
    symbol: Symbol,
    side: Option<OrderSide>,
    entry: Option<f64>,
    targets: Option<Vec<f64>>,
    timeframe: Option<String>,
    stop_loss: Option<f64>,
}
impl TradeIntent {
    pub fn builder(symbol: &Symbol) -> TradeIntentBuilder {
        TradeIntentBuilder {
            symbol: *symbol,
            side: None,
            entry: None,
            targets: None,
            timeframe: None,
            stop_loss: None,
        }
    }
}

impl TradeIntentBuilder {
    pub fn side(mut self, side: OrderSide) -> Self {
        self.side = Some(side);
        self
    }

    pub fn entry(mut self, entry: f64) -> Self {
        self.entry = Some(entry);
        self
    }

    pub fn stop_loss(mut self, stop_loss: f64) -> Self {
        self.stop_loss = Some(stop_loss);
        self
    }

    pub fn targets(mut self, targets: &[f64]) -> Self {
        self.targets = Some(targets.to_vec());
        self
    }

    pub fn timeframe(mut self, timeframe: &str) -> Self {
        self.timeframe = Some(timeframe.to_string());
        self
    }

    pub fn build(self) -> Result<TradeIntent, TradeIntentError> {
        Ok(TradeIntent {
            intent_id: Uuid::new_v4(),
            symbol: self.symbol,
            side: self.side.ok_or(TradeIntentError::MissingSide)?,
            entry: self.entry.ok_or(TradeIntentError::MissingEntry)?,
            targets: self.targets.ok_or(TradeIntentError::MissingTargets)?,
            timeframe: self.timeframe.ok_or(TradeIntentError::MissingTimeframe)?,
            stop_loss: self.stop_loss.ok_or(TradeIntentError::MissingStopLoss)?,
        })
    }
}
