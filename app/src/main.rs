use std::sync::Arc;

use dotenv::dotenv;
use telegram::{
    client::{ConnectClientReturnType, connect_client, handle_updates},
    dialogs::{get_dialogs, get_peer_by_bare_id, print_dialogs, print_peer_data},
};

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

    let from_peer = get_peer_by_bare_id(&client, 1649642332)
        .await?
        .expect("Could not find the from_peer in dialogs");

    let to_peer = get_peer_by_bare_id(&client, 5173657056)
        .await?
        .expect("Could not find the to_peer in dialogs");

    let tokens_peer = get_peer_by_bare_id(&client, 5144995821)
        .await?
        .expect("Could not find the to_peer in dialogs");


    // TODO
    let ignore_names_for_peer_senders = vec![
        String::from("Phanes"),
        String::from("Rick"),
    ];

    tokio::spawn(handle_updates(
        Arc::clone(&client),
        updates_receiver,
        Arc::clone(&bus),
    ));

    {
        // let dialogs = get_dialogs(&client).await?;
        // let _ = print_dialogs(&dialogs);
    };

    //TODO: Make so that forwarders can be added in runtime.

    tokio::spawn(forwarder::run(
        Arc::clone(&client),
        from_peer,
        to_peer,
        Arc::clone(&bus),
    ));

    tokio::spawn(token_addressess_forwarder::run(
        Arc::clone(&bus),
        Arc::clone(&client),
        tokens_peer,
    ));

    tokio::signal::ctrl_c().await?;

    Ok(())
}
