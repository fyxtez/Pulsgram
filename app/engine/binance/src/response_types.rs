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

    #[serde(default)]
    pub wallet_balance: Option<String>,

    #[serde(default)]
    pub unrealized_profit: Option<String>,

    #[serde(flatten)]
    pub _extra: serde_json::Value,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FuturesPosition {
    pub symbol: String,

    #[serde(default)]
    pub position_amt: Option<String>,

    #[serde(default)]
    pub entry_price: Option<String>,

    #[serde(default)]
    pub unrealized_profit: Option<String>,

    #[serde(flatten)]
    pub _extra: serde_json::Value,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FuturesAccountInfo {
    pub total_wallet_balance: String,

    #[serde(default)]
    pub available_balance: Option<String>,

    #[serde(default)]
    pub assets: Vec<FuturesAsset>,

    #[serde(default)]
    pub positions: Vec<FuturesPosition>,

    #[serde(flatten)]
    pub _extra: serde_json::Value,
}
#[derive(Debug, Deserialize)]
pub struct PositionModeResponse {
    #[serde(rename = "dualSidePosition")]
    pub dual_side_position: bool,
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
