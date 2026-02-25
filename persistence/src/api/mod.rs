mod routes;

use axum::Router;
use tokio::net::TcpListener;

use crate::{
    api::routes::{
        cors::build_cors_layer,
        middlewares::{request_id_middleware, request_logger, timeout_middleware},
        routes,
    },
    utils::shutdown_signal,
};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sqlx::Error as SqlxError;
use std::error::Error;

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

// TODO:
// match some_db_call.await {
//     Ok(val) => Ok(Json(val)),
//     Err(err) => Err(map_db_error(err)),
// }
pub fn map_db_error(err: SqlxError) -> Response {
    println!("DB Error: {}", err);

    match err {
        SqlxError::RowNotFound => (StatusCode::NOT_FOUND, "Resource not found").into_response(),

        SqlxError::Database(db_err) if db_err.code().as_deref() == Some("23505") => {
            (StatusCode::CONFLICT, "Duplicate entry").into_response()
        }

        _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response(),
    }
}
