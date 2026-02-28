use app_state::AppState;
use axum::{Extension, Json, http::StatusCode, response::IntoResponse};
use domain::types::{trade::TradeApproved, trade_intent::TradeIntent};
use std::sync::Arc;

use crate::dto::dev::DevTradeApprovedRequest;

pub async fn dev_trade_approved(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<DevTradeApprovedRequest>,
) -> impl IntoResponse {
    let intent = match TradeIntent::builder(&payload.symbol)
        .entry(payload.entry)
        .side(payload.side)
        .stop_loss(payload.stop_loss)
        .targets(&payload.targets)
        .timeframe(&payload.timeframe)
        .build()
    {
        Ok(intent) => intent,
        Err(error) => {
            eprintln!("Dev - TradeApproved error: {:?}", error);

            return (
                StatusCode::BAD_REQUEST,
                format!("Dev - TradeApproved - Wrong parameters: {}", error),
            );
        }
    };

    let approved: TradeApproved = intent.into();

    state
        .bus
        .publish(publisher::types::PulsgramEvent::TradeApproved(approved));

    (StatusCode::OK, "Dev Trade Approved".to_string())
}
