use crate::error::AppError;

mod bootstrap;
mod config;
mod error;
mod types;
mod utils;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let runtime = bootstrap::bootstrap().await?;
    bootstrap::run(runtime).await
}
