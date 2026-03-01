use crate::error::AppError;

pub fn create_reqwest_client() -> Result<reqwest::Client, AppError> {
    use std::time::Duration;

    let client = reqwest::Client::builder()
        .user_agent("Pulsgram/1.0")

        .connect_timeout(Duration::from_secs(5)) 
        // Network safety
        // How long is the waiting to establish TCP connection to Binance.
        // Includes DNS resolution, TCP & TLS handshake
        // Not having this makes request freeze, potentially for a long time.
        .timeout(Duration::from_secs(20)) 
        // Prevents long requests

        .pool_idle_timeout(Duration::from_secs(30))
        // Pool tuning
        // Stops connection leak 
        // How long unused connections stay alive in the pool.
        // HTTP clients reuse TCP connections for speed.
        // This cleans those up after 30 seconds if they are idle.
        .pool_max_idle_per_host(10) // Cleans dead sockets

        .tcp_keepalive(Duration::from_secs(60)) 
        // Keep connections alive
        // Sends periodic “I’m alive” signal on TCP connection.

        .build()?;

    Ok(client)
}

/// Returns the build version embedded at compile time.
///
/// During deployment:
///     BUILD_VERSION=<git_hash> cargo build
///
/// `option_env!("BUILD_VERSION")` reads that value at compile time
/// and hardcodes it into the binary.
///
/// At runtime:
/// - No environment variable is read
/// - No file is opened
/// - No systemd config is required
///
/// If BUILD_VERSION was not provided during compilation
/// (for example, manual `cargo build`), it falls back to "dev".
pub fn get_build_version() -> &'static str {
    option_env!("BUILD_VERSION").unwrap_or("dev")
}
