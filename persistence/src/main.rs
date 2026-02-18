mod api;
mod db;
mod queries;
use crate::db::{connect, health_check, run};
pub use queries::chats;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = connect(&database_url).await;

    println!("Connected to database!");

    health_check(&pool).await?;

    run(&pool).await?;

    api::start_api_server("127.0.0.1", 8180).await?;

    Ok(())
}
