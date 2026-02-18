mod ping;
mod cors;

use axum::{
    Extension, Router,
    body::Body,
    http::{Request, StatusCode},
    response::IntoResponse,
    routing::get,
};
use std::sync::Arc;

use crate::routes::{cors::build_cors_layer, ping::ping};

pub fn create(app_state: Arc<app_state::AppState>) -> Router {
    Router::new()
        .nest("/api/v1", routes())
        .layer(build_cors_layer())
        .layer(Extension(app_state))
        .fallback(fallback)
}

fn routes() -> Router {
    Router::new().merge(_routes())
}

fn _routes() -> Router {
    Router::new().route("/ping", get(ping))
}

pub async fn fallback(req: Request<Body>) -> impl IntoResponse {
    let path = req.uri().path();
    (
        StatusCode::NOT_FOUND,
        format!("That endpoint '{}' is not in our API.", path),
    )
}
