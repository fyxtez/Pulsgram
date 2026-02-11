mod routes;

use std::sync::Arc;

use app_state::AppState;
use tokio::net::TcpListener;

use crate::routes::create;

pub async fn start_api_server(address: &str, port: i32, app_state: Arc<AppState>) {
    let listener = TcpListener::bind(format!("{}:{}", address, port))
        .await
        .unwrap();

    let router = create(app_state);

    println!("API Server starting at port: {}", port);

    axum::serve(listener, router).await.unwrap();
}
