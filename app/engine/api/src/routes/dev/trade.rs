use app_state::AppState;
use axum::{Extension, http::StatusCode, response::IntoResponse};
use domain::{TradeApproved, TradeIntent};
use std::sync::Arc;

pub async fn dev_trade_approved(Extension(state): Extension<Arc<AppState>>) -> impl IntoResponse {
    let intent = TradeIntent::new("BTCUSDT".into(), true.into());

    let approved = TradeApproved {
        intent_id: intent.intent_id,
        symbol: "BTCUSDT".into(),
        side: intent.side,
    };

    state
        .bus
        .publish(publisher::types::PulsgramEvent::TradeApproved(approved));

    (StatusCode::OK, "Dev TradeApproved event published")
}
