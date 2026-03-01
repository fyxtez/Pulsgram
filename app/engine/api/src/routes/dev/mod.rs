use axum::Router;

use axum::routing::post;

pub mod trade;

pub fn routes() -> Router {
        Router::new().route("/trade-approved", post(trade::dev_trade_approved))
}
