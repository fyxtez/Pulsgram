mod routes;

use std::sync::Arc;

use app_state::AppState;
use tokio::net::TcpListener;

use crate::routes::create;

use std::error::Error;

pub async fn start_api_server(
    address: &str,
    port: i32,
    app_state: Arc<AppState>,
) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(format!("{}:{}", address, port)).await?;

    let router = create(app_state);

    println!("API Server starting at {}:{}", address, port);

    axum::serve(listener, router).await?;

    Ok(())
}
