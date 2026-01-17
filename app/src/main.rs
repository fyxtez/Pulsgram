use std::sync::Arc;

use dotenv::dotenv;
use telegram::client::{ConnectClientReturnType, connect_client, handle_updates};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let session_path = "plusgram.session";

    let ConnectClientReturnType {
        client,
        updates_receiver,
    } = connect_client(session_path).await?;

    let client = Arc::new(client);

    let bus = Arc::new(publisher::new_event_bus());

    tokio::spawn(handle_updates(
        Arc::clone(&client),
        updates_receiver,
        Arc::clone(&bus),
    ));

    tokio::spawn(test_listener::run(Arc::clone(&bus)));

    tokio::signal::ctrl_c().await?;

    Ok(())
}
