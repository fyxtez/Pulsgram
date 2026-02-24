mod routes;

use axum::Router;
use tokio::net::TcpListener;

use std::error::Error;

use crate::{
    api::routes::{
        cors::build_cors_layer,
        middlewares::{request_id_middleware, request_logger, timeout_middleware},
        routes,
    },
    utils::shutdown_signal,
};

pub async fn start_api_server(address: &str, port: i32) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(format!("{}:{}", address, port)).await?;

    //TODO: Test timeout midware & other ones
    let router = Router::new()
        .nest("/persistance", routes())
        .layer(build_cors_layer())
        .layer(axum::middleware::from_fn(timeout_middleware))
        .layer(axum::middleware::from_fn(request_logger))
        .layer(axum::middleware::from_fn(request_id_middleware));

    println!("API Server starting at {}:{}", address, port);

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}
