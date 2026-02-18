use axum::{Router, routing::get};

use crate::api::routes::{cors::build_cors_layer, ping::ping};

pub mod cors;
mod ping;

pub fn build_router() -> Router {
    Router::new()
        .nest("/persistance", routes())
        .layer(build_cors_layer())
}

fn routes() -> Router {
    Router::new().merge(_routes())
}

fn _routes() -> Router {
    Router::new().route("/ping", get(ping))
}
