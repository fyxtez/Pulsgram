use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FuturesCommissionRateResponse {
    pub symbol: String,
    pub maker_commission_rate: String,
    pub taker_commission_rate: String,
    pub rpi_commission_rate: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FuturesAsset {
    pub asset: String,
    pub wallet_balance: String,
    pub unrealized_profit: String,
    pub margin_balance: String,
    pub maint_margin: String,
    pub initial_margin: String,
    pub position_initial_margin: String,
    pub open_order_initial_margin: String,
    pub cross_wallet_balance: String,
    pub cross_un_pnl: String,
    pub available_balance: String,
    pub max_withdraw_amount: String,
    pub margin_available: Option<bool>,
    pub update_time: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FuturesPosition {
    pub symbol: String,
    pub initial_margin: String,
    pub maint_margin: String,
    pub unrealized_profit: String,
    pub position_initial_margin: String,
    pub open_order_initial_margin: String,
    pub leverage: String,
    pub isolated: bool,
    pub entry_price: String,
    pub max_notional: String,
    pub bid_notional: String,
    pub ask_notional: String,
    pub position_side: String,
    pub position_amt: String,
    pub update_time: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FuturesAccountInfo {
    pub assets: Vec<FuturesAsset>,
    pub positions: Vec<FuturesPosition>,

    pub available_balance: String,
    pub max_withdraw_amount: String,

    pub total_cross_un_pnl: String,
    pub total_cross_wallet_balance: String,
    pub total_initial_margin: String,
    pub total_maint_margin: String,
    pub total_margin_balance: String,
    pub total_open_order_initial_margin: String,
    pub total_position_initial_margin: String,
    pub total_unrealized_profit: String,
    pub total_wallet_balance: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct PositionModeResponse {
    pub code: i64,
    pub msg: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionRisk {
    pub symbol: String,
    pub position_side: String, // BOTH, LONG, SHORT
    pub position_amt: String,
    pub entry_price: String,
    pub break_even_price: String,
    pub mark_price: String,
    pub un_realized_profit: String,
    pub liquidation_price: String,
    pub isolated_margin: String,
    pub notional: String,
    pub margin_asset: String,
    pub isolated_wallet: String,
    pub initial_margin: String,
    pub maint_margin: String,
    pub position_initial_margin: String,
    pub open_order_initial_margin: String,
    pub adl: i32,
    pub bid_notional: String,
    pub ask_notional: String,
    pub update_time: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FuturesOrderResponse {
    pub client_order_id: String,
    pub cum_qty: String,
    pub cum_quote: String,
    pub executed_qty: String,
    pub order_id: i64,
    pub avg_price: String,
    pub orig_qty: String,
    pub price: String,
    pub reduce_only: bool,
    pub side: String,
    pub position_side: String,
    pub status: String,
    pub stop_price: String,
    pub close_position: bool,
    pub symbol: String,
    pub time_in_force: String,
    pub r#type: String,
    pub orig_type: String,
    pub update_time: i64,
    pub working_type: String,
    pub price_protect: bool,
    pub price_match: String,
    pub self_trade_prevention_mode: String,
    pub good_till_date: Option<i64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetLeverageResponse {
    pub leverage: u32,
    pub max_notional_value: String,
    pub symbol: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListenKeyResponse {
    pub listen_key: String,
}
