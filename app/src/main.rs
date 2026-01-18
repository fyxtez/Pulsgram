use std::sync::Arc;

use dotenv::dotenv;
use telegram::{client::{ConnectClientReturnType, connect_client, handle_updates}, peers::{get_dialogs, print_dialogs}};

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

    let dialogs = get_dialogs(&client).await?;

    let _ = print_dialogs(&dialogs);

    // tokio::spawn(forwarder::run(
    //     Arc::clone(&client),
    //     // "-1001649642332".into(),
    //     // telegram::client::get_peer_from_env(&client, "FORWARD_TO_PEER") .await?,
    //     Arc::clone(&bus),
    // ));

    tokio::signal::ctrl_c().await?;

    Ok(())
}
