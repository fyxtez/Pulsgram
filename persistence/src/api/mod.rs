mod routes;

use tokio::net::TcpListener;

use std::error::Error;

use crate::api::routes::build_router;

pub async fn start_api_server(address: &str, port: i32) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(format!("{}:{}", address, port)).await?;

    let router = build_router();

    println!("API Server starting at {}:{}", address, port);

    axum::serve(listener, router).await?;

    Ok(())
}
