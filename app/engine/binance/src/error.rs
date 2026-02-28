use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum BinanceError {
    Http(reqwest::Error),
    Json(serde_json::Error),
    MissingField(&'static str),
    Api (BinanceApiErrorResponse),
    InvalidInput(String),
}

#[derive(Debug, serde::Deserialize)]
pub struct BinanceApiErrorResponse {
    pub code: i64,
    pub msg: String,
}

impl Display for BinanceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BinanceError::Http(err) => write!(f, "HTTP error: {}", err),
            BinanceError::Json(err) => write!(f, "JSON error: {}", err),
            BinanceError::MissingField(field) => {
                write!(f, "Missing field in response: {}", field)
            }
           BinanceError::Api(api_err) => {
                write!(
                    f,
                    "Binance API error ({}): {}",
                    api_err.code,
                    api_err.msg
                )
            }

            BinanceError::InvalidInput(msg) => write!(f, "Binance API error: {}", msg),
        }
    }
}

impl Error for BinanceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            BinanceError::Http(err) => Some(err),
            BinanceError::Json(err) => Some(err),
            BinanceError::MissingField(_) => None,
            BinanceError::Api(_) => None,
            BinanceError::InvalidInput(_) => None,
        }
    }
}

impl From<reqwest::Error> for BinanceError {
    fn from(err: reqwest::Error) -> Self {
        BinanceError::Http(err)
    }
}

impl From<serde_json::Error> for BinanceError {
    fn from(err: serde_json::Error) -> Self {
        BinanceError::Json(err)
    }
}
