use std::collections::HashMap;

use domain::types::symbol::{Symbol, SymbolFilters};

use crate::{
    error::BinanceError,
    response_types::{ExchangeFilter, ExchangeInfoResponse, ExchangeSymbol},
};

pub fn extract_supported_filters(
    exchange_info: &ExchangeInfoResponse,
    supported_symbols: &Vec<Symbol>,
) -> Result<HashMap<Symbol, SymbolFilters>, BinanceError> {
    let mut map = HashMap::new();

    for symbol in supported_symbols {
        let symbol_str = symbol.to_string();

        let exchange_symbol = exchange_info
            .symbols
            .iter()
            .find(|s| s.symbol == symbol_str)
            .ok_or(BinanceError::InvalidInput(format!(
                "Symbol {} not found in exchangeInfo",
                symbol_str
            )))?;

        let filters = extract_filters_from_symbol(exchange_symbol)?;

        map.insert(*symbol, filters);
    }

    Ok(map)
}

pub fn extract_filters_from_symbol(symbol: &ExchangeSymbol) -> Result<SymbolFilters, BinanceError> {
    let mut step_size = None;
    let mut min_qty = None;
    let mut min_notional = None;
    let mut tick_size = None;

    for filter in &symbol.filters {
        match filter {
            ExchangeFilter::LotSize {
                min_qty: mq,
                step_size: ss,
                ..
            } => {
                min_qty = Some(mq.parse::<f64>().unwrap());
                step_size = Some(ss.parse::<f64>().unwrap());
            }

            ExchangeFilter::MinNotional { notional } => {
                min_notional = Some(notional.parse::<f64>().unwrap());
            }

            ExchangeFilter::PriceFilter { tick_size: ts } => {
                tick_size = Some(ts.parse::<f64>().unwrap());
            }

            _ => {}
        }
    }

    Ok(SymbolFilters {
        step_size: step_size.ok_or(BinanceError::MissingField("stepSize"))?,
        min_qty: min_qty.ok_or(BinanceError::MissingField("minQty"))?,
        min_notional: min_notional.unwrap_or(0.0),
        tick_size: tick_size.ok_or(BinanceError::MissingField("tickSize"))?,
    })
}
