use axum::{Router, routing::post};

#[cfg(not(feature = "production"))]
pub mod trade;

pub fn routes() -> Router {
    #[cfg(not(feature = "production"))]
    {
        return Router::new()
            .route("/trade-approve", post(trade::dev_trade_approved));
    }

    #[cfg(feature = "production")]
    {
        return Router::new();
    }
}