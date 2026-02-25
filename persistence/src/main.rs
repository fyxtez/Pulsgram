mod api;
mod db;
mod repositories;
mod utils;

use crate::{
    db::{connect, health_check, run},
    repositories::Repositories,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = connect(&database_url).await?;
    
    let _repos = Repositories::new(pool.clone());

    println!("Connected to database!");

    health_check(&pool).await?;

    run(&pool).await?;

    api::start_api_server("127.0.0.1", 8180).await?;

    println!("Server stopped accepting new connections.");

    pool.close().await;

    println!("Database pool closed.");

    Ok(())
}
