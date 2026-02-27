mod cors;
mod dev;
mod ping;

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
    let router = Router::new().route("/ping", get(ping));

    #[cfg(not(feature = "production"))]
    let router = router.nest("/dev", dev::routes());

    router
}

//TODO: Shared
pub async fn fallback(req: Request<Body>) -> impl IntoResponse {
    let path = req.uri().path();
    (
        StatusCode::NOT_FOUND,
        format!("Endpoint '{}' is not in our API.", path),
    )
}
