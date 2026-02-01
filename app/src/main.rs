mod state;
mod utils;

use api::start_api_server;
// use db::{connect, run_migrations};
use dotenv::dotenv;
use std::{collections::HashSet, sync::Arc};
use telegram::{
    client::{ConnectClientReturnType, connect_client, handle_updates},
    dialogs::{
        DialogData, DialogType, get_dialog_type, get_dialogs, get_peer_by_bare_id,
        peer_to_dialog_data, print_dialogs, print_peer_data,
    },
};
use telegram_types::Peer;

use crate::utils::{dump_dialogs_to_json, get_peer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let session_path = "plusgram.session";

    let ConnectClientReturnType {
        client,
        updates_receiver,
    } = connect_client(session_path).await?;

    println!("Telegram Client connected.");

    let client = Arc::new(client);

    let bus = Arc::new(publisher::new_event_bus());

    tokio::spawn(handle_updates(
        Arc::clone(&client),
        updates_receiver,
        Arc::clone(&bus),
    ));

    println!("Updates handler spawned.");

    let dialogs = get_dialogs(&client).await?;

    println!("Telegram dialogs loaded.");

    let from_peer = get_peer(&client, 1649642332).await?;
    let to_peer = get_peer(&client, 5173657056).await?;
    let tokens_peer = get_peer(&client, 5144995821).await?;
    let errors_peer = get_peer(&client, 3876244916).await?;

    println!("Peers fetched.");

    // TODO
    let _ignored_senders: HashSet<&'static str> = ["Phanes", "Rick"].into_iter().collect();

    let _ignored_peers: HashSet<&Peer> = HashSet::new();

    let dialogs_data = Arc::new(dashmap::DashMap::new());

    for dialog in dialogs {
        let peer = dialog.peer();
        let (id, dialog_data) = peer_to_dialog_data(peer);

        dialogs_data.insert(id, dialog_data);
    }

    // TODO: Ignore Ph & Ri

    // dump_dialogs_to_json(&dialogs_data, "dialogs.json").unwrap();

    let state = state::AppState { dialogs_data };

    let shared_state = Arc::new(state);

    // tokio::spawn(forwarder::run(
    //     Arc::clone(&client),
    //     from_peer,
    //     to_peer,
    //     Arc::clone(&bus),
    // ));

    // tokio::spawn(token_addressess_forwarder::run(
    //     Arc::clone(&bus),
    //     Arc::clone(&client),
    //     tokens_peer,
    //     _ignored_senders,
    //     _ignored_peers,
    // ));

    tokio::spawn(analysis::run(
        Arc::clone(&client),
        Arc::clone(&bus),
        shared_state.dialogs_data.clone(),
        errors_peer.clone(),
    ));

    // let _db = connect("postgres://pulsgram_user:pulsgram_user@localhost:5432/pulsgram_db").await.unwrap();

    // run_migrations("../migrations", db);

    start_api_server().await;

    tokio::signal::ctrl_c().await?;

    Ok(())
}
