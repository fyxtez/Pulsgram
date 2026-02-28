use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderSide {
    Buy,  // LONG
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
