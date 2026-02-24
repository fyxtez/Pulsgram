use axum::{Router, routing::get};

use crate::api::routes::ping::ping;

pub mod cors;
pub mod middlewares;
mod ping;

pub fn routes() -> Router {
    Router::new().merge(_routes())
}

fn _routes() -> Router {
    Router::new().route("/ping", get(ping))
}
