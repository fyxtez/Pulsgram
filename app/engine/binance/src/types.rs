use std::fmt;

pub enum OrderSide {
    Buy,
    Sell,
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
