use tokio::signal;

//TODO: Shared.
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
