use crate::error::AppError;

pub fn create_reqwest_client() -> Result<reqwest::Client, AppError> {
    let client = reqwest::Client::builder()
        .user_agent("Pulsgram/1.0")
        .timeout(std::time::Duration::from_secs(30))
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