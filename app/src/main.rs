use std::sync::Arc;

use telegram::client::{ConnectClientReturnType, connect_client, handle_updates};
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    dotenv().ok();

    let session_path = "plusgram.session";

    let ConnectClientReturnType { client, updates_receiver } = connect_client(session_path).await?;

    let client = Arc::new(client);

    tokio::spawn(handle_updates(
        Arc::clone(&client),
        updates_receiver,
    ));

    tokio::signal::ctrl_c().await?;

    Ok(())
}
