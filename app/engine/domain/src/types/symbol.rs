use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Symbol {
    BTC,
    ETH,
    SOL,
    XRP,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Symbol::BTC => "BTCUSDT",
            Symbol::ETH => "ETHUSDT",
            Symbol::SOL => "SOLUSDT",
            Symbol::XRP => "XRPUSDT",
        };

        write!(f, "{s}")
    }
}

// Enables calling .parse
impl FromStr for Symbol {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BTC" | "BTCUSDT" => Ok(Symbol::BTC),
            "ETH" | "ETHUSDT" => Ok(Symbol::ETH),
            "SOL" | "SOLUSDT" => Ok(Symbol::SOL),
            "XRP" | "XRPUSDT" => Ok(Symbol::XRP),
            _ => Err(format!("Invalid symbol: {s}")),
        }
    }
}

use std::convert::TryFrom;

impl TryFrom<&str> for Symbol {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}
