mod routes;

use std::sync::Arc;

use app_state::AppState;
use tokio::net::TcpListener;

use crate::routes::create;

//TODO: Clear this up by moving port, adddress,... to function parameters or config.
pub async fn start_api_server(address: &str, port: i32, app_state: Arc<AppState>) {
    let listener = TcpListener::bind(format!("{}:{}", address, port))
        .await
        .unwrap();

    let router = create(app_state);

    println!("API Server starting at port: {}", port);

    axum::serve(listener, router).await.unwrap();
}
