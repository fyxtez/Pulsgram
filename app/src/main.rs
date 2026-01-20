mod state;
mod utils;

use api::start_api_server;
use dotenv::dotenv;
use grammers_client::types::Peer;
use tokio::net::TcpListener;
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

    let from_peer = must_get_peer(&client, 1649642332, "from").await?;
    let to_peer = must_get_peer(&client, 5173657056, "to").await?;
    let tokens_peer = must_get_peer(&client, 5144995821, "tokens").await?;

    // TODO
    let _ignored_senders: HashSet<&'static str> = ["Phanes", "Rick"].into_iter().collect();

    let _ignored_peers: HashSet<&Peer> = HashSet::new();

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
        _ignored_senders,
        _ignored_peers,
    ));

    let state = state::AppState {
    };

    let shared_state = Arc::new(state);

    start_api_server().await;

    tokio::signal::ctrl_c().await?;

    Ok(())
}
