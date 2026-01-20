use axum::Router;
use tokio::net::TcpListener;
use axum::routing::get;

//TODO: Clear this up by moving port, adddress,... to function parameters or config.
pub async fn start_api_server() {
    let port = 8000;

    let bind_address = "127.0.0.1";

    let listener = TcpListener::bind(format!("{}:{}", bind_address, port))
        .await
        .unwrap();

    let router = Router::new().route("/ping", get(ping));

    axum::serve(listener, router).await.unwrap();
}

async fn ping() -> &'static str {
    "pong"
}
