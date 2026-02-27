use app_state::AppState;
use axum::{Extension, http::StatusCode, response::IntoResponse};
use domain::types::{
    order_side::OrderSide, symbol::Symbol, trade::TradeApproved, trade_intent::TradeIntent,
};
use std::sync::Arc;

pub async fn dev_trade_approved(Extension(state): Extension<Arc<AppState>>) -> impl IntoResponse {
    let intent = match TradeIntent::builder(&Symbol::BTC)
        .entry(222222.2)
        .side(OrderSide::Buy)
        .stop_loss(11111.1)
        .targets(&[])
        .timeframe("30m")
        .build()
    {
        Ok(intent) => intent,
        Err(error) => {
            eprintln!("Dev TradeApproved error: {:?}", error);

            return (
                StatusCode::BAD_REQUEST,
                format!("Dev TradeApproved - Wrong parameters: {}", error),
            );
        }
    };

    let approved: TradeApproved = intent.into();

    state
        .bus
        .publish(publisher::types::PulsgramEvent::TradeApproved(approved));

    (
        StatusCode::OK,
        "Dev TradeApproved - event published".to_string(),
    )
}
