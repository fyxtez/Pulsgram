use crate::error::AppError;

mod types;
mod bootstrap;
mod config;
mod error;
mod utils;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let runtime = bootstrap::bootstrap().await?;
    bootstrap::run(runtime).await
}
