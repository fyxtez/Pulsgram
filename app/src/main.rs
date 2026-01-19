mod utils;

use dotenv::dotenv;
use std::{collections::HashSet, sync::Arc};
use telegram::{
    client::{ConnectClientReturnType, connect_client, handle_updates},
    dialogs::{get_dialogs, get_peer_by_bare_id, print_dialogs, print_peer_data},
};

use crate::utils::must_get_peer;

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

    let from_peer = must_get_peer(&client, 1649642332, "from").await?;
    let to_peer = must_get_peer(&client, 5173657056, "to").await?;
    let tokens_peer = must_get_peer(&client, 5144995821, "tokens").await?;

    // TODO
    let _ignored_senders: HashSet<&'static str> = ["Phanes", "Rick"].into_iter().collect();

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
        _ignored_senders
    ));

    tokio::signal::ctrl_c().await?;

    Ok(())
}
