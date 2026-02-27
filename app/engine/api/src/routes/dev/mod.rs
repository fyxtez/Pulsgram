use axum::{Router, routing::post};

#[cfg(not(feature = "production"))]
pub mod trade;

pub fn routes() -> Router {
    #[cfg(not(feature = "production"))]
    {
        Router::new().route("/trade-approve", post(trade::dev_trade_approved))
    }

    #[cfg(feature = "production")]
    {
        Router::new()
    }
}
