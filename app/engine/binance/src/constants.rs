pub const TESTNET_FUTURES: &str = "https://testnet.binancefuture.com";
pub const TESTNET_SPOT: &str = "https://testnet.binance.vision";

pub const SPOT: &str = "https://api.binance.com";
pub const FUTURES: &str = "https://fapi.binance.com";

use std::fmt;

pub enum Symbol {
    BTC,
    SOL,
    ETH,
}

impl Symbol {
    pub fn as_str(&self) -> &'static str {
        match self {
            Symbol::BTC => "BTCUSDT",
            Symbol::SOL => "SOLUSDT",
            Symbol::ETH => "ETHUSDT",
        }
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
