use crate::errors::BinanceError;
use domain::types::symbol::SymbolFilters;

pub fn align_down(value: f64, step: f64) -> f64 {
    (value / step).floor() * step
}

pub fn align_up(value: f64, step: f64) -> f64 {
    (value / step).ceil() * step
}

fn precision_from_step(step: f64) -> usize {
    step.to_string()
        .split('.')
        .nth(1)
        .map(|s| s.len())
        .unwrap_or(0)
}

pub fn format_with_step(value: f64, step: f64) -> String {
    format!("{:.*}", precision_from_step(step), value)
}

pub fn validate_qty(filters: &SymbolFilters, qty: f64) -> Result<f64, BinanceError> {
    if qty < filters.min_qty {
        return Err(BinanceError::InvalidInput(format!(
            "Quantity {} below min_qty {}",
            qty, filters.min_qty
        )));
    }

    let aligned = align_down(qty, filters.step_size);
    if aligned <= 0.0 {
        return Err(BinanceError::InvalidInput(
            "Quantity invalid after alignment".into(),
        ));
    }

    Ok(aligned)
}

pub fn validate_price(filters: &SymbolFilters, price: f64) -> Result<f64, BinanceError> {
    let aligned = align_down(price, filters.tick_size);
    if aligned <= 0.0 {
        return Err(BinanceError::InvalidInput(
            "Price invalid after alignment".into(),
        ));
    }
    Ok(aligned)
}

pub fn validate_notional(
    filters: &SymbolFilters,
    qty: f64,
    price: f64,
) -> Result<(), BinanceError> {
    let notional = qty * price;
    if notional < filters.min_notional {
        return Err(BinanceError::InvalidInput(format!(
            "Notional {} below min_notional {}",
            notional, filters.min_notional
        )));
    }
    Ok(())
}
