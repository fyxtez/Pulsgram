use std::collections::HashMap;

use crate::{errors::BinanceError, transport::Transport};
use domain::types::symbol::{Symbol, SymbolFilters};

#[derive(Clone)]
pub struct BinanceClient {
    client: reqwest::Client,
    base_url: String,
    api_key: String,
    api_secret: String,
    symbol_filters: HashMap<Symbol, SymbolFilters>,
}

impl BinanceClient {
    pub fn new(client: reqwest::Client, base_url: &str, api_key: &str, api_secret: &str) -> Self {
        Self {
            client,
            base_url: base_url.to_string(),
            api_key: api_key.to_string(),
            api_secret: api_secret.to_string(),
            symbol_filters: HashMap::new(),
        }
    }

    pub(crate) fn transport(&self) -> Transport<'_> {
        Transport {
            client: &self.client,
            base_url: &self.base_url,
            api_key: &self.api_key,
            api_secret: &self.api_secret,
        }
    }

    pub fn set_symbol_filters(&mut self, filters: HashMap<Symbol, SymbolFilters>) {
        self.symbol_filters = filters;
    }

    pub fn filters(&self, symbol: Symbol) -> Result<&SymbolFilters, BinanceError> {
        self.symbol_filters
            .get(&symbol)
            .ok_or_else(|| BinanceError::InvalidInput(format!("Unknown symbol: {}", symbol)))
    }
}
