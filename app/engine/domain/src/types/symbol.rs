use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Symbol {
    BTC,
    ETH,
    SOL,
    XRP,
    BNB,
    TRX,
    ADA,
    ASTER,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Symbol::BTC => "BTCUSDT",
            Symbol::ETH => "ETHUSDT",
            Symbol::SOL => "SOLUSDT",
            Symbol::XRP => "XRPUSDT",
            Symbol::BNB => "BNBUSDT",
            Symbol::TRX => "TRXUSDT",
            Symbol::ADA => "ADAUSDT",
            Symbol::ASTER => "ASTERUSDT",
        };

        write!(f, "{s}")
    }
}

// Enables calling .parse
impl FromStr for Symbol {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_uppercase().as_str() {
            "BTC" | "BTCUSDT" => Ok(Symbol::BTC),
            "ETH" | "ETHUSDT" => Ok(Symbol::ETH),
            "SOL" | "SOLUSDT" => Ok(Symbol::SOL),
            "XRP" | "XRPUSDT" => Ok(Symbol::XRP),
            "BNB" | "BNBUSDT" => Ok(Symbol::BNB),
            "TRX" | "TRXUSDT" => Ok(Symbol::TRX),
            "ADA" | "ADAUSDT" => Ok(Symbol::ADA),
            "ASTER" | "ASTERUSDT" => Ok(Symbol::ASTER),
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

#[derive(Debug, Clone)]
pub struct SymbolFilters {
    pub step_size: f64,
    pub min_qty: f64,
    pub min_notional: f64,
    pub tick_size: f64,
}
