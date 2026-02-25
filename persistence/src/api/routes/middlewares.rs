// TODO: Shared
use axum::{
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::time::{Duration, Instant};
use uuid::Uuid;

use axum::extract::Request;

pub async fn timeout_middleware(req: Request, next: Next) -> Response {
    match tokio::time::timeout(Duration::from_secs(10), next.run(req)).await {
        Ok(response) => response,
        Err(_) => (StatusCode::REQUEST_TIMEOUT, "Request took too long").into_response(),
    }
}

pub async fn request_logger(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let start = Instant::now();

    let response = next.run(req).await;

    let duration = start.elapsed();

    println!(
        "request completed | method={} uri={} status={} duration_ms={}",
        method,
        uri,
        response.status(),
        duration.as_millis()
    );

    response
}

pub async fn request_id_middleware(req: Request, next: Next) -> Response {
    let request_id = Uuid::new_v4();
    println!("incoming request; request_id={}", request_id);
    let mut response = next.run(req).await;

    response
        .headers_mut()
        .insert("x-request-id", request_id.to_string().parse().unwrap());

    response
}
