use std::env::VarError;
use std::error::Error;
use std::fmt::{Display, Formatter};

use binance::error::BinanceError;
use telegram::errors::TelegramError;

#[derive(Debug)]
pub enum AppError {
    Config(VarError),
    Telegram(Box<TelegramError>),
    Binance(BinanceError),
    Io(std::io::Error),
    Reqwest(reqwest::Error),
    NotFound(&'static str),
    Other(String),
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Config(msg) => write!(f, "Config error: {}", msg),
            AppError::Telegram(msg) => write!(f, "Telegram error: {}", msg),
            AppError::Io(err) => write!(f, "IO error: {}", err),
            AppError::Reqwest(msg) => write!(f, "Reqwest error: {}", msg),
            AppError::Binance(msg) => write!(f, "Binance error: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::Other(msg) => write!(f, "Other Error: {}", msg),
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::Telegram(e) => Some(e),
            AppError::Binance(e) => Some(e),
            AppError::Io(e) => Some(e),
            AppError::Reqwest(e) => Some(e),
            AppError::Config(e) => Some(e),
            AppError::NotFound(_) => None,
            AppError::Other(_) => None,
        }
    }
}
impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::Reqwest(err)
    }
}

impl From<TelegramError> for AppError {
    fn from(err: TelegramError) -> Self {
        AppError::Telegram(Box::new(err))
    }
}

impl From<BinanceError> for AppError {
    fn from(err: BinanceError) -> Self {
        AppError::Binance(err)
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err)
    }
}
impl From<VarError> for AppError {
    fn from(err: VarError) -> Self {
        AppError::Config(err)
    }
}
