mod ping;
use std::sync::Arc;
use axum::{
    Extension, Router, body::Body, http::{Request, StatusCode}, response::IntoResponse, routing::get
};

use crate::routes::ping::ping;

pub fn create(app_state:Arc<app_state::AppState>) -> Router {
    let router = Router::new().nest("/api/v1", routes()).layer(Extension(app_state)).fallback(fallback);

    router
}

fn routes() -> Router {
    Router::new().merge(_routes())
}

fn _routes() -> Router {
    let router = Router::new().route("/ping", get(ping));
    router
}

pub async fn fallback(req: Request<Body>) -> impl IntoResponse {
    let path = req.uri().path();
    (
        StatusCode::NOT_FOUND,
        format!("That endpoint '{}' is not in our API.", path),
    )
}
