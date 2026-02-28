mod dto;
mod routes;
use tokio::signal;

use std::{io, sync::Arc};

use app_state::AppState;
use tokio::net::TcpListener;

use crate::routes::create;

pub async fn start_api_server(
    address: &str,
    port: i32,
    app_state: Arc<AppState>,
) -> Result<(), io::Error> {
    let listener = TcpListener::bind(format!("{}:{}", address, port)).await?;

    let api_prefix = "/api/v1";

    let router = create(app_state,api_prefix);

    println!("API Server starting at {}:{} with prefix: {}", address, port, api_prefix);

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

pub async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    // On Unix systems (Linux, macOS), listen for SIGTERM.
    // SIGTERM is what systemd, Docker, and Kubernetes send
    // when they want the process to shut down gracefully.
    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    // On non-Unix systems (e.g., Windows),
    // A future is created that never resolves.
    // That way only Ctrl+C will trigger shutdown.
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    // Wait for either Ctrl+C OR SIGTERM.
    // Whichever happens first will cause this function to continue
    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("Shutdown signal received.");
}
