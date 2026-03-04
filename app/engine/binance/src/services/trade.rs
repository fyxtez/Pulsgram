use domain::types::{order_side::OrderSide, symbol::Symbol};

use crate::{
    client::BinanceClient,
    errors::BinanceError,
    filters::quantize::{
        align_up, format_with_step, validate_notional, validate_price, validate_qty,
    },
    response_types::FuturesOrderResponse,
};

impl BinanceClient {
    pub async fn place_minimum_market_order(
        &self,
        symbol: Symbol,
        side: &OrderSide,
    ) -> Result<FuturesOrderResponse, BinanceError> {
        let filters = self.filters(symbol)?;

        let current_price = self.get_current_price(symbol).await?;

        let min_notional_qty = filters.min_notional / current_price;

        let raw = filters.min_qty.max(min_notional_qty);

        let qty = align_up(raw, filters.step_size);

        self.place_market_order(symbol, side, qty).await
    }

    pub async fn close_percentage(&self, symbol: Symbol, percent: f64) -> Result<(), BinanceError> {
        if percent <= 0.0 || percent > 100.0 {
            return Err(BinanceError::InvalidInput(
                "percent must be between 0 and 100".into(),
            ));
        }

        let positions = self.get_position_risk(Some(symbol)).await?;

        let pos = positions
            .into_iter()
            .find(|p| p.symbol == symbol.to_string())
            .ok_or_else(|| BinanceError::InvalidInput("Position not found".into()))?;

        let amt: f64 = pos.position_amt.parse().unwrap_or(0.0);

        if amt == 0.0 {
            return Ok(());
        }

        let raw = amt.abs() * percent / 100.0;

        if raw <= 0.0 {
            return self.close_full_position(symbol).await;
        }

        let side = if amt > 0.0 {
            OrderSide::Sell
        } else {
            OrderSide::Buy
        };

        self.place_market_order(symbol, &side, raw).await?;

        Ok(())
    }
    pub async fn close_full_position(&self, symbol: Symbol) -> Result<(), BinanceError> {
        let positions = self.get_position_risk(Some(symbol)).await?;

        let pos = positions
            .into_iter()
            .find(|p| p.symbol == symbol.to_string())
            .ok_or_else(|| BinanceError::InvalidInput("Position not found".into()))?;

        let amt: f64 = pos
            .position_amt
            .parse()
            .map_err(|_| BinanceError::InvalidInput("Invalid position amount".into()))?;

        if amt == 0.0 {
            return Ok(());
        }

        let side = if amt > 0.0 {
            OrderSide::Sell
        } else {
            OrderSide::Buy
        };

        self.place_market_order(symbol, &side, amt.abs()).await?;

        Ok(())
    }

    pub async fn place_market_order(
        &self,
        symbol: Symbol,
        side: &OrderSide,
        quantity: f64,
    ) -> Result<FuturesOrderResponse, BinanceError> {
        let filters = self.filters(symbol)?;
        let aligned_qty = validate_qty(filters, quantity)?;

        let quantity_str = format_with_step(aligned_qty, filters.step_size);

        self.place_market_order_raw(symbol, side, quantity_str)
            .await
    }

    pub async fn place_limit_order(
        &self,
        symbol: Symbol,
        side: &OrderSide,
        quantity: f64,
        price: f64,
    ) -> Result<FuturesOrderResponse, BinanceError> {
        let filters = self.filters(symbol)?;
        let aligned_qty = validate_qty(filters, quantity)?;

        let aligned_price = validate_price(filters, price)?;

        validate_notional(filters, aligned_qty, aligned_price)?;

        let quantity_str = format_with_step(aligned_qty, filters.step_size);
        let price_str = format_with_step(aligned_price, filters.tick_size);

        self.place_limit_order_raw(symbol, side, quantity_str, price_str)
            .await
    }
}
